pub const PHI: f32 = 1.618_034;

pub struct Scale {
    base: f32,
    k: f32,
}

impl Scale {
    #[must_use]
    pub fn base(mut self, base: f32) -> Self {
        self.base = base;
        self
    }

    #[must_use]
    pub fn k(mut self, k: f32) -> Self {
        self.k = k;
        self
    }

    #[must_use]
    pub fn compute(self) -> f32 {
        self.base * PHI.powf(self.k)
    }
}

impl Default for Scale {
    fn default() -> Self {
        Self { base: 1.0, k: 1.0 }
    }
}

#[must_use]
pub fn scale() -> Scale {
    Scale::default()
}
