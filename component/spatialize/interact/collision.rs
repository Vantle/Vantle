pub use error;

use bound::Bounded;
use intersect::Intersection;
use vector::Vector;

#[derive(Debug, Clone, Copy)]
pub struct Proximity {
    pub distance: f32,
    pub point: Vector,
}

impl Proximity {
    #[must_use]
    pub fn new(distance: f32, point: Vector) -> Self {
        Self { distance, point }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Event<L> {
    pub label: L,
    pub intersection: Intersection,
}

impl<L> Event<L> {
    #[must_use]
    pub fn new(label: L, intersection: Intersection) -> Self {
        Self {
            label,
            intersection,
        }
    }

    #[must_use]
    pub fn distance(&self) -> f32 {
        self.intersection.distance
    }
}

pub trait Labeled {
    type Label: Clone + Eq;

    fn label(&self) -> Self::Label;
}

pub trait Spatial: Bounded + Labeled {}

impl<T: Bounded + Labeled> Spatial for T {}

pub trait Raycast: Spatial {
    fn raycast(&self, ray: &ray::Ray) -> Option<Intersection>;
}

pub trait Proximal: Spatial {
    fn nearest(&self, point: Vector) -> Proximity;
}

pub trait Enclose: Spatial {
    fn contains(&self, point: Vector) -> bool;
}
