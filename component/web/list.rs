use element::Element;
use span::Span;

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
            children: span.elements,
        });
        self
    }

    #[must_use]
    pub fn feature(mut self, term: &str, description: &str) -> Self {
        self.items.push(Element::Tag {
            name: "li".into(),
            attributes: Vec::new(),
            children: vec![
                Element::Tag {
                    name: "strong".into(),
                    attributes: Vec::new(),
                    children: vec![Element::Text(term.into())],
                },
                Element::Text(description.into()),
            ],
        });
        self
    }

    #[must_use]
    pub fn glossary(mut self, term: &str, description: &str) -> Self {
        self.items.push(Element::Tag {
            name: "li".into(),
            attributes: Vec::new(),
            children: vec![
                Element::Tag {
                    name: "code".into(),
                    attributes: Vec::new(),
                    children: vec![Element::Text(term.into())],
                },
                Element::Text(description.into()),
            ],
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

    #[must_use]
    pub fn class(mut self, reference: class::Reference) -> Self {
        if let Some(Element::Tag { attributes, .. }) = self.items.last_mut() {
            element::merge(attributes, "class", reference.name());
        }
        self
    }

    #[must_use]
    pub fn attribute(mut self, name: &str, value: &str) -> Self {
        if let Some(Element::Tag { attributes, .. }) = self.items.last_mut() {
            attributes.push((name.into(), value.into()));
        }
        self
    }
}

impl Default for List {
    fn default() -> Self {
        Self::new()
    }
}
