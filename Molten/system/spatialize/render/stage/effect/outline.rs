use color::Color;
use proportion::PHI;

pub const DEFAULT: Color = Color {
    r: 0.987,
    g: 0.610,
    b: 0.144,
    a: 1.0,
};

#[derive(Debug, Clone, Copy)]
pub struct Outline {
    pub color: Color,
    pub width: f32,
}

impl PartialEq for Outline {
    fn eq(&self, other: &Self) -> bool {
        self.color == other.color && self.width.to_bits() == other.width.to_bits()
    }
}

impl Eq for Outline {}

impl std::hash::Hash for Outline {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.color.hash(state);
        self.width.to_bits().hash(state);
    }
}

impl Outline {
    #[must_use]
    pub fn new() -> Self {
        Self {
            color: DEFAULT,
            width: PHI,
        }
    }

    #[must_use]
    pub fn color(mut self, color: impl Into<Color>) -> Self {
        self.color = color.into();
        self
    }

    #[must_use]
    pub fn width(mut self, width: f32) -> Self {
        self.width = width;
        self
    }
}

impl Default for Outline {
    fn default() -> Self {
        Self::new()
    }
}
