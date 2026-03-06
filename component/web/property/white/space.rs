pub enum Space {
    Normal,
    Nowrap,
    Pre,
    Wrap,
}

impl Space {
    #[must_use]
    pub fn css(&self) -> &'static str {
        match self {
            Self::Normal => "normal",
            Self::Nowrap => "nowrap",
            Self::Pre => "pre",
            Self::Wrap => "pre-wrap",
        }
    }
}
