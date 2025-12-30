use bound::{Bound, Bounded};
use collision::{Enclose, Labeled, Proximal, Proximity, Raycast};
use cylinder::Cylinder as CylinderGeometry;
use hypergraph::Label;
use intersect::{Intersectable, Intersection};
use ray::Ray;
use region::Region;
use sphere::Sphere as SphereGeometry;
use vector::Vector;

pub struct Sphere {
    pub label: Label,
    pub geometry: SphereGeometry,
}

impl Sphere {
    #[must_use]
    pub fn new(label: Label, center: Vector, radius: f32) -> Self {
        Self {
            label,
            geometry: SphereGeometry::new(center, radius),
        }
    }
}

impl Bounded for Sphere {
    fn bound(&self) -> Bound {
        self.geometry.bound()
    }
}

impl Labeled for Sphere {
    type Label = Label;

    fn label(&self) -> Label {
        self.label
    }
}

impl Raycast for Sphere {
    fn raycast(&self, ray: &Ray) -> Option<Intersection> {
        self.geometry.intersect(ray)
    }
}

impl Proximal for Sphere {
    fn nearest(&self, point: Vector) -> Proximity {
        let direction = point - self.geometry.center;
        let distance = direction.magnitude();
        if distance < f32::EPSILON {
            let surface = self.geometry.center + Vector::new(self.geometry.radius, 0.0, 0.0);
            Proximity::new(self.geometry.radius, surface)
        } else {
            let normalized = direction.normalize();
            let surface = self.geometry.center + normalized * self.geometry.radius;
            let distance = (point - surface).magnitude();
            Proximity::new(distance, surface)
        }
    }
}

impl Enclose for Sphere {
    fn contains(&self, point: Vector) -> bool {
        let distance = (point - self.geometry.center).magnitude();
        distance <= self.geometry.radius
    }
}

pub struct Billboard {
    pub label: Label,
    pub geometry: Region,
}

impl Billboard {
    #[must_use]
    pub fn new(label: Label, position: Vector, width: f32, height: f32) -> Self {
        let center = Vector::new(
            position.x + width * 0.5,
            position.y - height * 0.5,
            position.z,
        );
        let extent = Vector::new(width * 0.5, height * 0.5, height * 0.5);
        Self {
            label,
            geometry: Region::new(center, extent),
        }
    }
}

impl Bounded for Billboard {
    fn bound(&self) -> Bound {
        self.geometry.bound()
    }
}

impl Labeled for Billboard {
    type Label = Label;

    fn label(&self) -> Label {
        self.label
    }
}

impl Raycast for Billboard {
    fn raycast(&self, ray: &Ray) -> Option<Intersection> {
        self.geometry.intersect(ray)
    }
}

impl Proximal for Billboard {
    fn nearest(&self, point: Vector) -> Proximity {
        let (lower, upper) = self.geometry.corners();
        let clamped = Vector::new(
            point.x.clamp(lower[0], upper[0]),
            point.y.clamp(lower[1], upper[1]),
            point.z.clamp(lower[2], upper[2]),
        );
        let distance = (point - clamped).magnitude();
        Proximity::new(distance, clamped)
    }
}

impl Enclose for Billboard {
    fn contains(&self, point: Vector) -> bool {
        let (lower, upper) = self.geometry.corners();
        point.x >= lower[0]
            && point.x <= upper[0]
            && point.y >= lower[1]
            && point.y <= upper[1]
            && point.z >= lower[2]
            && point.z <= upper[2]
    }
}

pub struct Cylinder {
    pub label: Label,
    pub geometry: CylinderGeometry,
}

impl Cylinder {
    #[must_use]
    pub fn new(label: Label, source: Vector, target: Vector, radius: f32) -> Self {
        Self {
            label,
            geometry: CylinderGeometry::new(source, target, radius),
        }
    }
}

impl Bounded for Cylinder {
    fn bound(&self) -> Bound {
        self.geometry.bound()
    }
}

impl Labeled for Cylinder {
    type Label = Label;

    fn label(&self) -> Label {
        self.label
    }
}

impl Raycast for Cylinder {
    fn raycast(&self, ray: &Ray) -> Option<Intersection> {
        self.geometry.intersect(ray)
    }
}

impl Proximal for Cylinder {
    fn nearest(&self, point: Vector) -> Proximity {
        let axis = &self.geometry.axis;
        let offset = point - axis.origin;
        let t = offset.dot(&axis.direction).clamp(0.0, axis.length);
        let projection = axis.origin + axis.direction * t;
        let radial = point - projection;
        let distance = radial.magnitude();

        let surface = if distance < f32::EPSILON {
            projection + Vector::new(self.geometry.radius, 0.0, 0.0)
        } else {
            projection + radial.normalize() * self.geometry.radius
        };

        let result = (point - surface).magnitude();
        Proximity::new(result, surface)
    }
}

pub enum Geometry {
    Sphere(Sphere),
    Cylinder(Cylinder),
    Billboard(Billboard),
}

impl Bounded for Geometry {
    fn bound(&self) -> Bound {
        match self {
            Self::Sphere(s) => s.bound(),
            Self::Cylinder(c) => c.bound(),
            Self::Billboard(b) => b.bound(),
        }
    }
}

impl Labeled for Geometry {
    type Label = Label;

    fn label(&self) -> Label {
        match self {
            Self::Sphere(s) => s.label,
            Self::Cylinder(c) => c.label,
            Self::Billboard(b) => b.label,
        }
    }
}

impl Raycast for Geometry {
    fn raycast(&self, ray: &Ray) -> Option<Intersection> {
        match self {
            Self::Sphere(s) => s.raycast(ray),
            Self::Cylinder(c) => c.raycast(ray),
            Self::Billboard(b) => b.raycast(ray),
        }
    }
}

impl Proximal for Geometry {
    fn nearest(&self, point: Vector) -> Proximity {
        match self {
            Self::Sphere(s) => s.nearest(point),
            Self::Cylinder(c) => c.nearest(point),
            Self::Billboard(b) => b.nearest(point),
        }
    }
}

impl Enclose for Geometry {
    fn contains(&self, point: Vector) -> bool {
        match self {
            Self::Sphere(s) => s.contains(point),
            Self::Cylinder(_) => false,
            Self::Billboard(b) => b.contains(point),
        }
    }
}
