use color::Color;
use effect::{Effect, Effected};
use vector::Vector;

pub trait Primitive: Sized {
    type Commands: IntoIterator<Item = Geometry>;

    fn commands(&self) -> Self::Commands;

    fn effect(self, effect: impl Into<Effect>) -> Effected<Self> {
        Effected {
            inner: self,
            effects: vec![effect.into()],
        }
    }

    fn effects(self, effects: impl IntoIterator<Item = Effect>) -> Effected<Self> {
        Effected {
            inner: self,
            effects: effects.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Geometry {
    Sphere(Sphere),
    Cylinder(Cylinder),
    Cone(Cone),
    Label(Label),
}

#[derive(Debug, Clone, Copy)]
pub struct Sphere {
    pub center: Vector,
    pub radius: f32,
    pub color: Color,
}

impl PartialEq for Sphere {
    fn eq(&self, other: &Self) -> bool {
        self.center == other.center
            && self.radius.to_bits() == other.radius.to_bits()
            && self.color == other.color
    }
}

impl Eq for Sphere {}

impl std::hash::Hash for Sphere {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.center.hash(state);
        self.radius.to_bits().hash(state);
        self.color.hash(state);
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Cylinder {
    pub source: Vector,
    pub target: Vector,
    pub radius: f32,
    pub color: Color,
}

impl PartialEq for Cylinder {
    fn eq(&self, other: &Self) -> bool {
        self.source == other.source
            && self.target == other.target
            && self.radius.to_bits() == other.radius.to_bits()
            && self.color == other.color
    }
}

impl Eq for Cylinder {}

impl std::hash::Hash for Cylinder {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.source.hash(state);
        self.target.hash(state);
        self.radius.to_bits().hash(state);
        self.color.hash(state);
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Cone {
    pub base: Vector,
    pub apex: Vector,
    pub radius: f32,
    pub color: Color,
}

impl PartialEq for Cone {
    fn eq(&self, other: &Self) -> bool {
        self.base == other.base
            && self.apex == other.apex
            && self.radius.to_bits() == other.radius.to_bits()
            && self.color == other.color
    }
}

impl Eq for Cone {}

impl std::hash::Hash for Cone {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.base.hash(state);
        self.apex.hash(state);
        self.radius.to_bits().hash(state);
        self.color.hash(state);
    }
}

#[derive(Debug, Clone)]
pub struct Label {
    pub content: String,
    pub position: Vector,
    pub size: f32,
    pub color: Color,
}

#[derive(Debug, Clone, Copy)]
pub struct Arrow {
    pub source: Vector,
    pub target: Vector,
    pub radius: f32,
    pub color: Color,
}

impl PartialEq for Arrow {
    fn eq(&self, other: &Self) -> bool {
        self.source == other.source
            && self.target == other.target
            && self.radius.to_bits() == other.radius.to_bits()
            && self.color == other.color
    }
}

impl Eq for Arrow {}

impl std::hash::Hash for Arrow {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.source.hash(state);
        self.target.hash(state);
        self.radius.to_bits().hash(state);
        self.color.hash(state);
    }
}

impl Sphere {
    #[must_use]
    pub fn new(center: impl Into<Vector>, radius: f32, color: impl Into<Color>) -> Self {
        Self {
            center: center.into(),
            radius,
            color: color.into(),
        }
    }
}

impl Cylinder {
    #[must_use]
    pub fn new(
        source: impl Into<Vector>,
        target: impl Into<Vector>,
        radius: f32,
        color: impl Into<Color>,
    ) -> Self {
        Self {
            source: source.into(),
            target: target.into(),
            radius,
            color: color.into(),
        }
    }
}

impl Cone {
    #[must_use]
    pub fn new(
        base: impl Into<Vector>,
        apex: impl Into<Vector>,
        radius: f32,
        color: impl Into<Color>,
    ) -> Self {
        Self {
            base: base.into(),
            apex: apex.into(),
            radius,
            color: color.into(),
        }
    }
}

impl Label {
    #[must_use]
    pub fn new(
        content: impl Into<String>,
        position: impl Into<Vector>,
        size: f32,
        color: impl Into<Color>,
    ) -> Self {
        Self {
            content: content.into(),
            position: position.into(),
            size,
            color: color.into(),
        }
    }
}

impl Arrow {
    #[must_use]
    pub fn new(
        source: impl Into<Vector>,
        target: impl Into<Vector>,
        radius: f32,
        color: impl Into<Color>,
    ) -> Self {
        Self {
            source: source.into(),
            target: target.into(),
            radius,
            color: color.into(),
        }
    }
}

impl Primitive for Sphere {
    type Commands = std::iter::Once<Geometry>;
    fn commands(&self) -> Self::Commands {
        std::iter::once(Geometry::Sphere(*self))
    }
}

impl Primitive for Cylinder {
    type Commands = std::iter::Once<Geometry>;
    fn commands(&self) -> Self::Commands {
        std::iter::once(Geometry::Cylinder(*self))
    }
}

impl Primitive for Cone {
    type Commands = std::iter::Once<Geometry>;
    fn commands(&self) -> Self::Commands {
        std::iter::once(Geometry::Cone(*self))
    }
}

impl Primitive for Label {
    type Commands = std::iter::Once<Geometry>;
    fn commands(&self) -> Self::Commands {
        std::iter::once(Geometry::Label(self.clone()))
    }
}

impl Primitive for Arrow {
    type Commands = std::vec::IntoIter<Geometry>;
    fn commands(&self) -> Self::Commands {
        let axis = self.target - self.source;
        let length = axis.magnitude();

        if length < 0.001 {
            return Vec::new().into_iter();
        }

        let direction = axis.normalize();
        let head = length.min(self.radius * 4.0);
        let junction = self.target - direction * head;

        let cylinder = Cylinder::new(self.source, junction, self.radius, self.color);
        let cone = Cone::new(junction, self.target, self.radius * 2.0, self.color);

        vec![Geometry::Cylinder(cylinder), Geometry::Cone(cone)].into_iter()
    }
}
