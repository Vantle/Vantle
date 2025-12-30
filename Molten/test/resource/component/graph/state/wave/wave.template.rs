use component::graph::state::particle::Particle;
use component::graph::state::wave::Wave;

fn monochromatic(data: Particle<String>) -> Wave<String> {
    Wave::monochromatic(data)
}

fn polychromatic(data: Particle<String>, multiplicity: usize) -> Wave<String> {
    Wave::polychromatic(data, multiplicity)
}

fn new(particles: Vec<(Particle<String>, usize)>) -> Wave<String> {
    Wave::new(particles.into_iter().collect())
}

fn empty(wave: &Wave<String>) -> bool {
    wave.empty()
}

mod from {
    use super::{Particle, Wave};

    pub fn slice(particles: Vec<Particle<String>>) -> Wave<String> {
        Wave::from(particles.as_slice())
    }
}
