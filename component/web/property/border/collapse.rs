pub enum Collapse {
    Separate,
    Collapse,
}

impl Collapse {
    #[must_use]
    pub fn css(&self) -> &'static str {
        match self {
            Self::Separate => "separate",
            Self::Collapse => "collapse",
        }
    }
}
