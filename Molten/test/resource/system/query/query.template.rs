use component::graph::state::particle::Particle;
use component::graph::state::wave::Wave;
use system::query::{Polyset, Ranked, Set};

pub mod particle {
    use super::{Particle, Ranked, Set};

    #[must_use]
    pub fn subset(source: Particle<String>, basis: Particle<String>) -> bool {
        source.subset(&basis).is_some()
    }

    #[must_use]
    pub fn superset(source: Particle<String>, basis: Particle<String>) -> bool {
        source.superset(&basis).is_some()
    }

    #[must_use]
    pub fn joint(source: Particle<String>, basis: Particle<String>) -> bool {
        source.joint(&basis).is_some()
    }

    #[must_use]
    pub fn disjoint(source: Particle<String>, basis: Particle<String>) -> bool {
        source.disjoint(&basis).is_some()
    }

    #[must_use]
    pub fn isomorphic(source: Particle<String>, basis: Particle<String>) -> bool {
        source.isomorphic(&basis).is_some()
    }

    #[must_use]
    pub fn rank(source: Particle<String>) -> usize {
        source.rank()
    }

    #[must_use]
    pub fn empty(source: Particle<String>) -> bool {
        source.empty()
    }
}

pub mod wave {
    use super::{Polyset, Ranked, Set, Wave};

    #[must_use]
    pub fn subset(source: Wave<String>, basis: Wave<String>) -> bool {
        source.subset(&basis).is_some()
    }

    #[must_use]
    pub fn superset(source: Wave<String>, basis: Wave<String>) -> bool {
        source.superset(&basis).is_some()
    }

    #[must_use]
    pub fn rank(source: Wave<String>) -> usize {
        source.rank()
    }

    #[must_use]
    pub fn diverges(source: Wave<String>, basis: Wave<String>) -> Vec<Wave<String>> {
        source.diverges(&basis)
    }
}
