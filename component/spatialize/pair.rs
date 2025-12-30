#[derive(Debug, Clone, Copy, Default)]
pub struct Pair {
    pub x: f32,
    pub y: f32,
}

impl Pair {
    #[must_use]
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    #[must_use]
    pub fn array(&self) -> [f32; 2] {
        [self.x, self.y]
    }
}

impl From<[f32; 2]> for Pair {
    fn from(array: [f32; 2]) -> Self {
        Self {
            x: array[0],
            y: array[1],
        }
    }
}

impl From<(f32, f32)> for Pair {
    fn from(tuple: (f32, f32)) -> Self {
        Self {
            x: tuple.0,
            y: tuple.1,
        }
    }
}

impl From<Pair> for [f32; 2] {
    fn from(pair: Pair) -> Self {
        pair.array()
    }
}

impl From<Pair> for (f32, f32) {
    fn from(pair: Pair) -> Self {
        (pair.x, pair.y)
    }
}
