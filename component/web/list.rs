use element::Element;
use span::{Fragment, Span};

pub struct List {
    pub items: Vec<Element>,
}

impl List {
    #[must_use]
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    #[must_use]
    pub fn item(mut self, f: impl FnOnce(Span) -> Span) -> Self {
        let span = f(Span::new());
        self.items.push(Element::Tag {
            name: "li".into(),
            attributes: Vec::new(),
            children: vec![Element::Span(span.fragments)],
        });
        self
    }

    #[must_use]
    pub fn feature(mut self, term: &str, description: &str) -> Self {
        self.items.push(Element::Tag {
            name: "li".into(),
            attributes: Vec::new(),
            children: vec![Element::Span(vec![
                Fragment::Bold(term.into()),
                Fragment::Text(description.into()),
            ])],
        });
        self
    }

    #[must_use]
    pub fn glossary(mut self, term: &str, description: &str) -> Self {
        self.items.push(Element::Tag {
            name: "li".into(),
            attributes: Vec::new(),
            children: vec![Element::Span(vec![
                Fragment::Code(term.into()),
                Fragment::Text(description.into()),
            ])],
        });
        self
    }

    #[must_use]
    pub fn plain(mut self, text: &str) -> Self {
        self.items.push(Element::Tag {
            name: "li".into(),
            attributes: Vec::new(),
            children: vec![Element::Text(text.into())],
        });
        self
    }
}

impl Default for List {
    fn default() -> Self {
        Self::new()
    }
}
