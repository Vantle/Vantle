use component::graph::state::particle::Particle;
use std::collections::BTreeMap;

fn fundamental(data: String) -> Particle<String> {
    Particle::fundamental(data)
}

fn new(elements: BTreeMap<String, usize>) -> Particle<String> {
    Particle::new(elements)
}

mod from {
    use super::Particle;

    pub fn slice(elements: Vec<String>) -> Particle<String> {
        Particle::from(elements.as_slice())
    }
}
