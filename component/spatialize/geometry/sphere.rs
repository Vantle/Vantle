use bound::{Bound, Bounded};
use intersect::{Intersectable, solve};
use ray::Ray;
use vector::Vector;

#[derive(Debug, Clone, Copy)]
pub struct Sphere {
    pub center: Vector,
    pub radius: f32,
}

impl Sphere {
    #[must_use]
    pub fn new(center: Vector, radius: f32) -> Self {
        Self { center, radius }
    }
}

impl Bounded for Sphere {
    fn bound(&self) -> Bound {
        let c = self.center.array();
        let r = self.radius;
        Bound::new(
            [c[0] - r, c[1] - r, c[2] - r],
            [c[0] + r, c[1] + r, c[2] + r],
        )
    }
}

impl Intersectable for Sphere {
    fn intersect(&self, ray: &Ray) -> Option<intersect::Intersection> {
        let oc = ray.origin - self.center;

        let a = ray.direction.dot(&ray.direction);
        let b = 2.0 * oc.dot(&ray.direction);
        let c = oc.dot(&oc) - self.radius * self.radius;

        solve::nearest(a, b, c).map(|t| {
            let point = ray.origin + ray.direction * t;
            let normal = (point - self.center).normalize();
            intersect::Intersection::new(t, point, normal)
        })
    }
}
