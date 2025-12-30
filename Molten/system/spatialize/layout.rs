use std::collections::BTreeMap;

use error::{Error, Result};
use hypergraph::Label;
use rstar::{AABB, RTree, RTreeObject};
use space::Space;
use vector::Vector;

#[derive(Debug, Clone, Copy)]
pub struct Tuning {
    pub center: f32,
    pub damping: f32,
    pub cooling: f32,
}

impl Default for Tuning {
    fn default() -> Self {
        Self {
            center: 0.001,
            damping: 0.85,
            cooling: 0.99,
        }
    }
}

struct Entry {
    label: Label,
    position: Vector,
}

impl Entry {
    fn new(label: Label, position: Vector) -> Self {
        Self { label, position }
    }
}

impl RTreeObject for Entry {
    type Envelope = AABB<[f32; 3]>;

    fn envelope(&self) -> Self::Envelope {
        AABB::from_point(self.position.array())
    }
}

pub struct Simulation {
    positions: BTreeMap<Label, Vector>,
    velocities: BTreeMap<Label, Vector>,
    temperature: f32,
    configuration: Tuning,
}

impl Simulation {
    #[must_use]
    #[expect(clippy::cast_precision_loss)]
    pub fn new(space: &Space, bounds: f32, configuration: Tuning) -> Self {
        let mut positions = BTreeMap::new();
        let mut velocities = BTreeMap::new();

        let count = space.count();
        let boundary = scale::glyph() * 2.0;
        let radius = scale::radius(bounds).max(boundary);

        for (i, node) in space.nodes().enumerate() {
            let t = (i as f32) / (count.max(1) as f32);
            let inclination = (1.0 - 2.0 * t).acos();
            let azimuth = 2.0 * std::f32::consts::PI * (i as f32) * proportion::scale().compute();

            let position = Vector::new(
                radius * inclination.sin() * azimuth.cos(),
                radius * inclination.cos(),
                radius * inclination.sin() * azimuth.sin(),
            );

            positions.insert(node.label(), position);
            velocities.insert(node.label(), Vector::default());
        }

        Self {
            positions,
            velocities,
            temperature: 1.0,
            configuration,
        }
    }

    pub fn step(&mut self, space: &Space, bounds: f32) -> Result<()> {
        for connector in space.edges() {
            for &source in connector.sources() {
                if !self.positions.contains_key(&source) {
                    return Err(Error::Missing { label: source });
                }
            }
            for &sink in connector.sinks() {
                if !self.positions.contains_key(&sink) {
                    return Err(Error::Missing { label: sink });
                }
            }
        }

        let labels = self.positions.keys().copied().collect::<Vec<_>>();
        let mut forces = BTreeMap::<Label, Vector>::new();
        let count = self.positions.len();

        let repulsion = scale::repulsion(count);
        let attraction = scale::attraction(count);

        for &label in &labels {
            forces.insert(label, Vector::default());
        }

        self.repel(&labels, &mut forces, repulsion, bounds);

        for connector in space.edges() {
            for &source in connector.sources() {
                for &sink in connector.sinks() {
                    let delta = self.positions[&sink] - self.positions[&source];
                    let force = delta * attraction;

                    if let Some(f) = forces.get_mut(&source) {
                        *f = *f + force;
                    }
                    if let Some(f) = forces.get_mut(&sink) {
                        *f = *f - force;
                    }
                }
            }
        }

        for &label in &labels {
            let p = self.positions[&label];

            if let Some(f) = forces.get_mut(&label) {
                *f = *f - p * self.configuration.center;
            }
        }

        for &label in &labels {
            let force = forces[&label];

            if let Some(v) = self.velocities.get_mut(&label) {
                *v = (*v + force * self.temperature) * self.configuration.damping;
            }

            if let Some(p) = self.positions.get_mut(&label) {
                let v = self.velocities[&label];
                let limit = bounds * 0.5;
                *p = Vector::new(
                    (p.x + v.x).clamp(-limit, limit),
                    (p.y + v.y).clamp(-limit, limit),
                    (p.z + v.z).clamp(-limit, limit),
                );
            }
        }

        self.temperature *= self.configuration.cooling;
        Ok(())
    }

    #[expect(clippy::cast_precision_loss)]
    fn repel(
        &self,
        labels: &[Label],
        forces: &mut BTreeMap<Label, Vector>,
        repulsion: f32,
        bounds: f32,
    ) {
        let entries = labels
            .iter()
            .filter_map(|&l| self.positions.get(&l).map(|&p| Entry::new(l, p)))
            .collect::<Vec<_>>();

        let tree = RTree::bulk_load(entries);
        let count = self.positions.len();
        let radius = bounds / (count as f32).sqrt();

        for &label in labels {
            let p = self.positions[&label];
            let array = p.array();
            let lower = [array[0] - radius, array[1] - radius, array[2] - radius];
            let upper = [array[0] + radius, array[1] + radius, array[2] + radius];
            let envelope = AABB::from_corners(lower, upper);

            for entry in tree.locate_in_envelope(&envelope) {
                if entry.label.0 > label.0 {
                    Self::apply(p, entry.position, label, entry.label, forces, repulsion);
                }
            }
        }
    }

    fn apply(
        pa: Vector,
        pb: Vector,
        a: Label,
        b: Label,
        forces: &mut BTreeMap<Label, Vector>,
        repulsion: f32,
    ) {
        let delta = pa - pb;
        let distance = delta.magnitude().max(1.0);
        let force = repulsion / (distance * distance);
        let direction = delta / distance * force;

        if let Some(f) = forces.get_mut(&a) {
            *f = *f + direction;
        }
        if let Some(f) = forces.get_mut(&b) {
            *f = *f - direction;
        }
    }

    #[must_use]
    pub fn converged(&self) -> bool {
        self.temperature < 0.01
    }

    pub fn reset(&mut self) {
        self.temperature = 1.0;
    }

    #[must_use]
    pub fn positions(&self) -> &BTreeMap<Label, Vector> {
        &self.positions
    }

    #[must_use]
    pub fn position(&self, label: Label) -> Option<Vector> {
        self.positions.get(&label).copied()
    }
}
