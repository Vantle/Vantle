pub struct Table {
    pub headers: Vec<String>,
    pub rows: Vec<Vec<String>>,
}

impl Table {
    #[must_use]
    pub fn new() -> Self {
        Self {
            headers: Vec::new(),
            rows: Vec::new(),
        }
    }

    #[must_use]
    pub fn header<const N: usize>(mut self, columns: [&str; N]) -> Self {
        self.headers = columns.iter().map(|c| (*c).into()).collect::<Vec<_>>();
        self
    }

    #[must_use]
    pub fn row<const N: usize>(mut self, cells: [&str; N]) -> Self {
        self.rows
            .push(cells.iter().map(|c| (*c).into()).collect::<Vec<_>>());
        self
    }
}

impl Default for Table {
    fn default() -> Self {
        Self::new()
    }
}
