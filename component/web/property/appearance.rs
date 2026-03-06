pub enum Appearance {
    None,
}

impl Appearance {
    #[must_use]
    pub fn css(&self) -> &'static str {
        match self {
            Self::None => "none",
        }
    }
}
