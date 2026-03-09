use element::Element;

pub struct Span {
    pub elements: Vec<Element>,
}

impl Span {
    #[must_use]
    pub fn new() -> Self {
        Self {
            elements: Vec::new(),
        }
    }

    #[must_use]
    pub fn text(mut self, content: &str) -> Self {
        self.elements.push(Element::Text(content.into()));
        self
    }

    #[must_use]
    pub fn bold(mut self, content: &str) -> Self {
        self.elements.push(Element::Tag {
            name: "strong".into(),
            attributes: Vec::new(),
            children: vec![Element::Text(content.into())],
        });
        self
    }

    #[must_use]
    pub fn italic(mut self, content: &str) -> Self {
        self.elements.push(Element::Tag {
            name: "em".into(),
            attributes: Vec::new(),
            children: vec![Element::Text(content.into())],
        });
        self
    }

    #[must_use]
    pub fn code(mut self, content: &str) -> Self {
        self.elements.push(Element::Tag {
            name: "code".into(),
            attributes: Vec::new(),
            children: vec![Element::Text(content.into())],
        });
        self
    }

    #[must_use]
    pub fn link(mut self, href: &str, f: impl FnOnce(Span) -> Span) -> Self {
        let inner = f(Span::new());
        self.elements.push(Element::Tag {
            name: "a".into(),
            attributes: vec![("href".into(), href.into())],
            children: inner.elements,
        });
        self
    }

    #[must_use]
    pub fn class(mut self, reference: class::Reference) -> Self {
        if let Some(Element::Tag { attributes, .. }) = self.elements.last_mut() {
            element::merge(attributes, "class", reference.name());
        }
        self
    }

    #[must_use]
    pub fn attribute(mut self, name: &str, value: &str) -> Self {
        if let Some(Element::Tag { attributes, .. }) = self.elements.last_mut() {
            attributes.push((name.into(), value.into()));
        }
        self
    }
}

impl Default for Span {
    fn default() -> Self {
        Self::new()
    }
}
