pub enum Decoration {
    None,
    Underline,
    LineThrough,
}

impl Decoration {
    #[must_use]
    pub fn css(&self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Underline => "underline",
            Self::LineThrough => "line-through",
        }
    }
}
