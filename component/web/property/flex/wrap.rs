pub enum Wrap {
    Nowrap,
    Wrap,
}

impl Wrap {
    #[must_use]
    pub fn css(&self) -> &'static str {
        match self {
            Self::Nowrap => "nowrap",
            Self::Wrap => "wrap",
        }
    }
}
