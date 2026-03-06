pub enum Weight {
    W400,
    W500,
    W600,
    W700,
}

impl Weight {
    #[must_use]
    pub fn css(&self) -> &'static str {
        match self {
            Self::W400 => "400",
            Self::W500 => "500",
            Self::W600 => "600",
            Self::W700 => "700",
        }
    }
}
