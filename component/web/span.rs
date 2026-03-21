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
    pub fn class(mut self, reference: reference::Reference) -> Self {
        if let Some(Element::Tag { attributes, .. }) = self.elements.last_mut() {
            for word in reference.words() {
                element::merge(attributes, "class", word);
            }
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

    #[must_use]
    pub fn data(self, reference: attribute::Reference, value: &str) -> Self {
        self.attribute(reference.name(), value)
    }

    #[must_use]
    pub fn identifier(self, value: &str) -> Self {
        self.attribute("id", value)
    }

    #[must_use]
    pub fn label(self, value: &str) -> Self {
        self.attribute("aria-label", value)
    }

    #[must_use]
    pub fn current(self, value: &str) -> Self {
        self.attribute("aria-current", value)
    }

    #[must_use]
    pub fn inline(self, value: &str) -> Self {
        self.attribute("style", value)
    }
}

impl Default for Span {
    fn default() -> Self {
        Self::new()
    }
}
