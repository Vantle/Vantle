use component::graph::state::particle::Particle as Particulate;

use query::{Ranked, Set as QuerySet};
use scale::Scaled;
use translate::Set;

pub trait Particle: Set + QuerySet + Ranked + Scaled {
    type Element;
}

impl<T: Clone + Eq + Ord> Particle for Particulate<T> {
    type Element = T;
}
