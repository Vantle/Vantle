use bound::{Bound, Bounded};
use intersect::Intersectable;
use ray::Ray;
use tolerance::EPSILON;
use vector::Vector;

#[derive(Debug, Clone, Copy)]
pub struct Region {
    pub center: Vector,
    pub extent: Vector,
}

impl Region {
    #[must_use]
    pub fn new(center: Vector, extent: Vector) -> Self {
        Self { center, extent }
    }

    #[must_use]
    pub fn corners(&self) -> ([f32; 3], [f32; 3]) {
        let c = self.center.array();
        let e = self.extent.array();
        (
            [c[0] - e[0], c[1] - e[1], c[2] - e[2]],
            [c[0] + e[0], c[1] + e[1], c[2] + e[2]],
        )
    }
}

impl Bounded for Region {
    fn bound(&self) -> Bound {
        let (lower, upper) = self.corners();
        Bound::new(lower, upper)
    }
}

impl Intersectable for Region {
    fn intersect(&self, ray: &Ray) -> Option<intersect::Intersection> {
        let (lower, upper) = self.corners();

        let mut tmin = 0.0f32;
        let mut tmax = f32::MAX;
        let mut normal_min = Vector::new(0.0, 0.0, 0.0);
        let mut normal_max = Vector::new(0.0, 0.0, 0.0);

        let origin = ray.origin.array();
        let direction = ray.direction.array();

        for i in 0..3 {
            if direction[i].abs() < EPSILON {
                if origin[i] < lower[i] || origin[i] > upper[i] {
                    return None;
                }
            } else {
                let inv = 1.0 / direction[i];
                let mut t0 = (lower[i] - origin[i]) * inv;
                let mut t1 = (upper[i] - origin[i]) * inv;

                let (mut n0, mut n1) = match i {
                    0 => (Vector::new(-1.0, 0.0, 0.0), Vector::new(1.0, 0.0, 0.0)),
                    1 => (Vector::new(0.0, -1.0, 0.0), Vector::new(0.0, 1.0, 0.0)),
                    _ => (Vector::new(0.0, 0.0, -1.0), Vector::new(0.0, 0.0, 1.0)),
                };

                if inv < 0.0 {
                    std::mem::swap(&mut t0, &mut t1);
                    std::mem::swap(&mut n0, &mut n1);
                }

                if t0 > tmin {
                    tmin = t0;
                    normal_min = n0;
                }
                if t1 < tmax {
                    tmax = t1;
                    normal_max = n1;
                }

                if tmax < tmin {
                    return None;
                }
            }
        }

        let (t, normal) = if tmin > 0.0 {
            (tmin, normal_min)
        } else if tmax > 0.0 {
            (tmax, normal_max)
        } else {
            return None;
        };

        let point = ray.origin + ray.direction * t;
        Some(intersect::Intersection::new(t, point, normal))
    }
}
