use collision::{Event, Proximal, Proximity, Raycast};
use intersect::Intersectable;
use ray::Ray;
use region::Region;
use rstar::{AABB, RTree, RTreeObject};
use vector::Vector;

pub struct Entry<F: Raycast> {
    pub form: F,
    bound: AABB<[f32; 3]>,
}

impl<F: Raycast> Entry<F> {
    #[must_use]
    pub fn new(form: F) -> Self {
        let b = form.bound();
        Self {
            form,
            bound: AABB::from_corners(b.minimum, b.maximum),
        }
    }
}

impl<F: Raycast> RTreeObject for Entry<F> {
    type Envelope = AABB<[f32; 3]>;

    fn envelope(&self) -> Self::Envelope {
        self.bound
    }
}

pub struct Index<F: Raycast> {
    tree: RTree<Entry<F>>,
}

impl<F: Raycast> Index<F> {
    #[must_use]
    pub fn new() -> Self {
        Self { tree: RTree::new() }
    }

    #[must_use]
    pub fn build(forms: Vec<F>) -> Self {
        let entries = forms.into_iter().map(Entry::new).collect::<Vec<_>>();
        Self {
            tree: RTree::bulk_load(entries),
        }
    }

    pub fn insert(&mut self, form: F) {
        self.tree.insert(Entry::new(form));
    }

    #[must_use]
    pub fn raycast(&self, ray: &Ray) -> Option<Event<F::Label>> {
        let mut closest = None::<Event<F::Label>>;

        for entry in &self.tree {
            let envelope = entry.envelope();
            let lower = envelope.lower();
            let upper = envelope.upper();
            let center = Vector::new(
                (lower[0] + upper[0]) * 0.5,
                (lower[1] + upper[1]) * 0.5,
                (lower[2] + upper[2]) * 0.5,
            );
            let extent = Vector::new(
                (upper[0] - lower[0]) * 0.5,
                (upper[1] - lower[1]) * 0.5,
                (upper[2] - lower[2]) * 0.5,
            );

            if Region::new(center, extent).intersect(ray).is_none() {
                continue;
            }

            if let Some(intersection) = entry.form.raycast(ray) {
                match &closest {
                    None => closest = Some(Event::new(entry.form.label(), intersection)),
                    Some(current) if intersection.distance < current.intersection.distance => {
                        closest = Some(Event::new(entry.form.label(), intersection));
                    }
                    _ => {}
                }
            }
        }

        closest
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.tree.size()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.tree.size() == 0
    }
}

impl<F: Raycast + Proximal> Index<F> {
    #[must_use]
    pub fn nearest(&self, point: Vector) -> Option<(F::Label, Proximity)> {
        let mut closest = None::<(F::Label, Proximity)>;

        for entry in &self.tree {
            let proximity = entry.form.nearest(point);

            match &closest {
                None => closest = Some((entry.form.label(), proximity)),
                Some((_, current)) if proximity.distance < current.distance => {
                    closest = Some((entry.form.label(), proximity));
                }
                _ => {}
            }
        }

        closest
    }
}

impl<F: Raycast> Default for Index<F> {
    fn default() -> Self {
        Self::new()
    }
}
