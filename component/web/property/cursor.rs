pub enum Cursor {
    Pointer,
    Default,
    Text,
}

impl Cursor {
    #[must_use]
    pub fn css(&self) -> &'static str {
        match self {
            Self::Pointer => "pointer",
            Self::Default => "default",
            Self::Text => "text",
        }
    }
}
