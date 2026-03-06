pub enum Overflow {
    Visible,
    Hidden,
    Scroll,
    Auto,
}

impl Overflow {
    #[must_use]
    pub fn css(&self) -> &'static str {
        match self {
            Self::Visible => "visible",
            Self::Hidden => "hidden",
            Self::Scroll => "scroll",
            Self::Auto => "auto",
        }
    }
}
