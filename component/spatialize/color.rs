#[repr(C)]
#[derive(Debug, Clone, Copy, Default, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    #[must_use]
    pub fn rgba(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }

    #[must_use]
    pub fn rgb(r: f32, g: f32, b: f32) -> Self {
        Self::rgba(r, g, b, 1.0)
    }

    #[must_use]
    pub fn transparent() -> Self {
        Self::rgba(0.0, 0.0, 0.0, 0.0)
    }

    #[must_use]
    #[expect(clippy::cast_precision_loss)]
    pub fn hex(value: u32) -> Self {
        let r = ((value >> 24) & 0xFF) as f32 / 255.0;
        let g = ((value >> 16) & 0xFF) as f32 / 255.0;
        let b = ((value >> 8) & 0xFF) as f32 / 255.0;
        let a = (value & 0xFF) as f32 / 255.0;
        Self::rgba(r, g, b, a)
    }

    #[must_use]
    pub fn lerp(self, other: Self, t: f32) -> Self {
        Self {
            r: self.r + (other.r - self.r) * t,
            g: self.g + (other.g - self.g) * t,
            b: self.b + (other.b - self.b) * t,
            a: self.a + (other.a - self.a) * t,
        }
    }

    #[must_use]
    pub fn array(&self) -> [f32; 4] {
        [self.r, self.g, self.b, self.a]
    }
}

impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        self.r.to_bits() == other.r.to_bits()
            && self.g.to_bits() == other.g.to_bits()
            && self.b.to_bits() == other.b.to_bits()
            && self.a.to_bits() == other.a.to_bits()
    }
}

impl Eq for Color {}

impl std::hash::Hash for Color {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.r.to_bits().hash(state);
        self.g.to_bits().hash(state);
        self.b.to_bits().hash(state);
        self.a.to_bits().hash(state);
    }
}

impl From<[f32; 4]> for Color {
    fn from(array: [f32; 4]) -> Self {
        Self {
            r: array[0],
            g: array[1],
            b: array[2],
            a: array[3],
        }
    }
}

impl From<[f32; 3]> for Color {
    fn from(array: [f32; 3]) -> Self {
        Self::rgb(array[0], array[1], array[2])
    }
}

impl From<Color> for [f32; 4] {
    fn from(color: Color) -> Self {
        color.array()
    }
}
