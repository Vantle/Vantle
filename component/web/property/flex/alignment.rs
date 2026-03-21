pub enum Alignment {
    Center,
    Start,
    End,
    Stretch,
    Baseline,
    Between,
    Around,
    Evenly,
}

impl Alignment {
    #[must_use]
    pub fn css(&self) -> &'static str {
        match self {
            Self::Center => "center",
            Self::Start => "flex-start",
            Self::End => "flex-end",
            Self::Stretch => "stretch",
            Self::Baseline => "baseline",
            Self::Between => "space-between",
            Self::Around => "space-around",
            Self::Evenly => "space-evenly",
        }
    }
}
