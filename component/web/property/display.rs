pub enum Display {
    Block,
    Flex,
    Grid,
    Inline,
    None,
}

impl Display {
    #[must_use]
    pub fn css(&self) -> &'static str {
        match self {
            Self::Block => "block",
            Self::Flex => "flex",
            Self::Grid => "grid",
            Self::Inline => "inline-block",
            Self::None => "none",
        }
    }
}
