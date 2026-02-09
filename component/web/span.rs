pub struct Span {
    pub fragments: Vec<Fragment>,
}

pub enum Fragment {
    Text(String),
    Bold(String),
    Italic(String),
    Code(String),
    Link { href: String, text: String },
}

impl Span {
    #[must_use]
    pub fn new() -> Self {
        Self {
            fragments: Vec::new(),
        }
    }

    #[must_use]
    pub fn text(mut self, content: &str) -> Self {
        self.fragments.push(Fragment::Text(content.into()));
        self
    }

    #[must_use]
    pub fn bold(mut self, content: &str) -> Self {
        self.fragments.push(Fragment::Bold(content.into()));
        self
    }

    #[must_use]
    pub fn italic(mut self, content: &str) -> Self {
        self.fragments.push(Fragment::Italic(content.into()));
        self
    }

    #[must_use]
    pub fn code(mut self, content: &str) -> Self {
        self.fragments.push(Fragment::Code(content.into()));
        self
    }

    #[must_use]
    pub fn link(mut self, href: &str, text: &str) -> Self {
        self.fragments.push(Fragment::Link {
            href: href.into(),
            text: text.into(),
        });
        self
    }
}

impl Default for Span {
    fn default() -> Self {
        Self::new()
    }
}
