pub enum Style {
    Normal,
    Italic,
}

impl Style {
    #[must_use]
    pub fn css(&self) -> &'static str {
        match self {
            Self::Normal => "normal",
            Self::Italic => "italic",
        }
    }
}
