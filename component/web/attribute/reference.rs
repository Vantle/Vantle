#[derive(Clone, Copy)]
pub struct Reference(pub &'static str);

impl Reference {
    #[must_use]
    pub fn name(self) -> &'static str {
        self.0
    }

    #[must_use]
    pub fn selector(self) -> String {
        format!("[{}]", self.0)
    }
}

impl std::fmt::Display for Reference {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.0)
    }
}
