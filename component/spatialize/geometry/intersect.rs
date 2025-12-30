pub use solve;

use ray::Ray;
use vector::Vector;

#[derive(Debug, Clone, Copy)]
pub struct Intersection {
    pub distance: f32,
    pub point: Vector,
    pub normal: Vector,
}

impl Intersection {
    #[must_use]
    pub fn new(distance: f32, point: Vector, normal: Vector) -> Self {
        Self {
            distance,
            point,
            normal,
        }
    }
}

pub trait Intersectable {
    fn intersect(&self, ray: &Ray) -> Option<Intersection>;
}
