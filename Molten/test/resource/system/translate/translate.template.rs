use component::graph::state::particle::Particle;
use component::graph::state::wave::Wave;
use system::translate::Set;

pub mod particle {
    use super::{Particle, Set};

    #[must_use]
    pub fn join(source: Particle<String>, basis: Particle<String>) -> Option<Particle<String>> {
        source.join(&basis)
    }

    #[must_use]
    pub fn intersect(
        source: Particle<String>,
        basis: Particle<String>,
    ) -> Option<Particle<String>> {
        source.intersect(&basis)
    }

    #[must_use]
    pub fn diverge(source: Particle<String>, basis: Particle<String>) -> Option<Particle<String>> {
        source.diverge(&basis)
    }
}

pub mod wave {
    use super::{Set, Wave};

    #[must_use]
    pub fn join(source: Wave<String>, basis: Wave<String>) -> Option<Wave<String>> {
        source.join(&basis)
    }

    #[must_use]
    pub fn intersect(source: Wave<String>, basis: Wave<String>) -> Option<Wave<String>> {
        source.intersect(&basis)
    }

    #[must_use]
    pub fn diverge(source: Wave<String>, basis: Wave<String>) -> Option<Wave<String>> {
        source.diverge(&basis)
    }
}
