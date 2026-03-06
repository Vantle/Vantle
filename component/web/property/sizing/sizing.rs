pub enum Box {
    Content,
    Border,
}

impl Box {
    #[must_use]
    pub fn css(&self) -> &'static str {
        match self {
            Self::Content => "content-box",
            Self::Border => "border-box",
        }
    }
}
