use tolerance::LENGTH;
use vector::Vector;

#[derive(Debug, Clone, Copy)]
pub struct Segment {
    pub origin: Vector,
    pub direction: Vector,
    pub length: f32,
}

impl Segment {
    #[must_use]
    pub fn new(source: Vector, target: Vector) -> Self {
        let delta = target - source;
        let length = delta.magnitude();
        let direction = if length > LENGTH {
            delta.normalize()
        } else {
            Vector::new(0.0, 0.0, 1.0)
        };
        Self {
            origin: source,
            direction,
            length,
        }
    }

    #[must_use]
    pub fn target(&self) -> Vector {
        self.origin + self.direction * self.length
    }
}
