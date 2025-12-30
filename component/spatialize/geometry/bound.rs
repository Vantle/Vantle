#[derive(Debug, Clone, Copy)]
pub struct Bound {
    pub minimum: [f32; 3],
    pub maximum: [f32; 3],
}

pub trait Bounded {
    fn bound(&self) -> Bound;
}

impl Bound {
    #[must_use]
    pub fn new(a: [f32; 3], b: [f32; 3]) -> Self {
        Self {
            minimum: [a[0].min(b[0]), a[1].min(b[1]), a[2].min(b[2])],
            maximum: [a[0].max(b[0]), a[1].max(b[1]), a[2].max(b[2])],
        }
    }

    #[must_use]
    pub fn center(&self) -> [f32; 3] {
        [
            (self.minimum[0] + self.maximum[0]) * 0.5,
            (self.minimum[1] + self.maximum[1]) * 0.5,
            (self.minimum[2] + self.maximum[2]) * 0.5,
        ]
    }

    #[must_use]
    pub fn size(&self) -> [f32; 3] {
        [
            self.maximum[0] - self.minimum[0],
            self.maximum[1] - self.minimum[1],
            self.maximum[2] - self.minimum[2],
        ]
    }

    #[must_use]
    pub fn contains(&self, point: [f32; 3]) -> bool {
        point[0] >= self.minimum[0]
            && point[0] <= self.maximum[0]
            && point[1] >= self.minimum[1]
            && point[1] <= self.maximum[1]
            && point[2] >= self.minimum[2]
            && point[2] <= self.maximum[2]
    }
}
