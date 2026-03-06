pub enum Align {
    Left,
    Center,
    Right,
    Justify,
}

impl Align {
    #[must_use]
    pub fn css(&self) -> &'static str {
        match self {
            Self::Left => "left",
            Self::Center => "center",
            Self::Right => "right",
            Self::Justify => "justify",
        }
    }
}
