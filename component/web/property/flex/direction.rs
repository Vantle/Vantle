pub enum Direction {
    Row,
    Column,
}

impl Direction {
    #[must_use]
    pub fn css(&self) -> &'static str {
        match self {
            Self::Row => "row",
            Self::Column => "column",
        }
    }
}
