use observe::trace;

use component::graph::state::particle::Particle;
use component::graph::state::wave::Wave;

pub trait Scaled {
    type Value;
    type Magnitude;
    fn scale(&self, basis: &Self::Value) -> Self::Magnitude;
}

impl<T: Clone + Eq + Ord> Scaled for Particle<T> {
    type Value = T;
    type Magnitude = usize;

    #[trace(channels = [core])]
    fn scale(&self, basis: &Self::Value) -> Self::Magnitude {
        self.elements.get(basis).copied().unwrap_or(0)
    }
}

impl<T: Clone + Eq + Ord> Scaled for Wave<T> {
    type Value = Particle<T>;
    type Magnitude = usize;

    #[trace(channels = [core])]
    fn scale(&self, basis: &Self::Value) -> Self::Magnitude {
        self.particles.get(basis).copied().unwrap_or(0)
    }
}
