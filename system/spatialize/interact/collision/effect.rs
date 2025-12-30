use collision::{Event, Raycast};
use event::{Sink, Source};
use index::Index;
use ray::Ray;

type Signal<L> = Option<Event<L>>;
type Receiver<L> = Box<dyn Sink<Signal<L>>>;

pub struct Effect<F: Raycast>
where
    F::Label: Clone + Eq,
{
    index: Index<F>,
    current: Signal<F::Label>,
    sinks: Vec<Receiver<F::Label>>,
}

impl<F: Raycast> Effect<F>
where
    F::Label: Clone + Eq,
{
    #[must_use]
    pub fn new() -> Self {
        Self {
            index: Index::new(),
            current: None,
            sinks: Vec::new(),
        }
    }

    pub fn react(&mut self, forms: Vec<F>) {
        self.index = Index::build(forms);
    }

    #[must_use]
    pub fn raycast(&mut self, ray: &Ray) -> Option<F::Label> {
        let hit = self.index.raycast(ray);

        if hit.as_ref().map(|h| &h.label) != self.current.as_ref().map(|c| &c.label) {
            self.current = hit;
            self.act(&self.current.clone());
        }

        self.current.as_ref().map(|h| h.label.clone())
    }

    #[must_use]
    pub fn current(&self) -> Option<&Event<F::Label>> {
        self.current.as_ref()
    }
}

impl<F: Raycast> Default for Effect<F>
where
    F::Label: Clone + Eq,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<F: Raycast> Source<Signal<F::Label>> for Effect<F>
where
    F::Label: Clone + Eq,
{
    fn interact(&mut self, sink: Receiver<F::Label>) {
        self.sinks.push(sink);
    }

    fn act(&mut self, event: &Signal<F::Label>) {
        for sink in &mut self.sinks {
            sink.react(event);
        }
    }
}
