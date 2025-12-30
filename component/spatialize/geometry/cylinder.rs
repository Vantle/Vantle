use bound::{Bound, Bounded};
use intersect::{Intersectable, solve};
use ray::Ray;
use segment::Segment;
use tolerance::LENGTH;
use vector::Vector;

#[derive(Debug, Clone, Copy)]
pub struct Cylinder {
    pub axis: Segment,
    pub radius: f32,
}

impl Cylinder {
    #[must_use]
    pub fn new(source: Vector, target: Vector, radius: f32) -> Self {
        Self {
            axis: Segment::new(source, target),
            radius,
        }
    }
}

impl Bounded for Cylinder {
    fn bound(&self) -> Bound {
        let s = self.axis.origin.array();
        let t = self.axis.target().array();
        let r = self.radius;
        Bound::new(
            [s[0].min(t[0]) - r, s[1].min(t[1]) - r, s[2].min(t[2]) - r],
            [s[0].max(t[0]) + r, s[1].max(t[1]) + r, s[2].max(t[2]) + r],
        )
    }
}

impl Intersectable for Cylinder {
    fn intersect(&self, ray: &Ray) -> Option<intersect::Intersection> {
        if self.axis.length < LENGTH {
            return None;
        }

        let oc = ray.origin - self.axis.origin;

        let da = ray.direction.dot(&self.axis.direction);
        let oa = oc.dot(&self.axis.direction);

        let dp = ray.direction - self.axis.direction * da;
        let op = oc - self.axis.direction * oa;

        let a = dp.dot(&dp);
        let b = 2.0 * dp.dot(&op);
        let c = op.dot(&op) - self.radius * self.radius;

        solve::nearest(a, b, c).and_then(|t| {
            let point = ray.origin + ray.direction * t;
            let projection = (point - self.axis.origin).dot(&self.axis.direction);

            (projection >= 0.0 && projection <= self.axis.length).then(|| {
                let base = self.axis.origin + self.axis.direction * projection;
                let normal = (point - base).normalize();
                intersect::Intersection::new(t, point, normal)
            })
        })
    }
}
