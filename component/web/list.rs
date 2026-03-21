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
    pub fn feature(self, term: &str, description: &str) -> Self {
        self.item(|s| s.bold(term).text(description))
    }

    #[must_use]
    pub fn glossary(self, term: &str, description: &str) -> Self {
        self.item(|s| s.code(term).text(description))
    }

    #[must_use]
    pub fn plain(self, text: &str) -> Self {
        self.item(|s| s.text(text))
    }

    #[must_use]
    pub fn class(mut self, reference: reference::Reference) -> Self {
        if let Some(Element::Tag { attributes, .. }) = self.items.last_mut() {
            for word in reference.words() {
                element::merge(attributes, "class", word);
            }
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

impl Default for List {
    fn default() -> Self {
        Self::new()
    }
}
