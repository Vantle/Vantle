pub enum Transform {
    None,
    Uppercase,
    Lowercase,
    Capitalize,
}

impl Transform {
    #[must_use]
    pub fn css(&self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Uppercase => "uppercase",
            Self::Lowercase => "lowercase",
            Self::Capitalize => "capitalize",
        }
    }
}
