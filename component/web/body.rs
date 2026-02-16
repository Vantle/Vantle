use element::{Element, Language, Source};
use span::Span;
use table::Table;

pub struct Body {
    pub elements: Vec<Element>,
    pending: Vec<(String, String)>,
}

impl Body {
    #[must_use]
    pub fn new() -> Self {
        Self {
            elements: Vec::new(),
            pending: Vec::new(),
        }
    }

    #[must_use]
    pub fn element(mut self, tag: &str, f: impl FnOnce(Body) -> Body) -> Self {
        let inner = f(Body::new());
        self.elements.push(Element::Tag {
            name: tag.into(),
            attributes: inner.pending,
            children: inner.elements,
        });
        self
    }

    #[must_use]
    pub fn text(mut self, content: &str) -> Self {
        self.elements.push(Element::Text(content.into()));
        self
    }

    #[must_use]
    pub fn span(mut self, f: impl FnOnce(Span) -> Span) -> Self {
        let span = f(Span::new());
        self.elements.push(Element::Span(span.fragments));
        self
    }

    #[must_use]
    pub fn html(mut self, raw: &str) -> Self {
        self.elements.push(Element::Raw(raw.into()));
        self
    }

    #[must_use]
    pub fn class(mut self, name: &str) -> Self {
        if let Some(existing) = self.pending.iter_mut().find(|(k, _)| k == "class") {
            existing.1.push(' ');
            existing.1.push_str(name);
        } else {
            self.pending.push(("class".into(), name.into()));
        }
        self
    }

    #[must_use]
    pub fn attribute(mut self, name: &str, value: &str) -> Self {
        self.pending.push((name.into(), value.into()));
        self
    }

    #[must_use]
    pub fn code(mut self, name: &str, language: Language) -> Self {
        self.elements.push(Element::Code {
            source: Source::File(name.into()),
            language,
        });
        self
    }

    #[must_use]
    pub fn literal(mut self, source: &str, language: Language) -> Self {
        self.elements.push(Element::Code {
            source: Source::Inline(source.into()),
            language,
        });
        self
    }

    #[must_use]
    pub fn highlight(mut self, html: &str, language: Language) -> Self {
        self.elements.push(Element::Tag {
            name: "div".into(),
            attributes: vec![
                ("class".into(), "code-block".into()),
                ("data-language".into(), language.name().into()),
            ],
            children: vec![Element::Raw(html.into())],
        });
        self
    }

    #[must_use]
    pub fn inject(mut self, name: &str) -> Self {
        self.elements.push(Element::Inject { name: name.into() });
        self
    }

    #[must_use]
    pub fn table(mut self, f: impl FnOnce(Table) -> Table) -> Self {
        let table = f(Table::new());
        let mut rows = Vec::new();

        if !table.headers.is_empty() {
            let header_cells = table
                .headers
                .into_iter()
                .map(|h| Element::Tag {
                    name: "th".into(),
                    attributes: Vec::new(),
                    children: vec![Element::Text(h)],
                })
                .collect::<Vec<_>>();
            rows.push(Element::Tag {
                name: "thead".into(),
                attributes: Vec::new(),
                children: vec![Element::Tag {
                    name: "tr".into(),
                    attributes: Vec::new(),
                    children: header_cells,
                }],
            });
        }

        let body_rows = table
            .rows
            .into_iter()
            .map(|row| {
                let cells = row
                    .into_iter()
                    .map(|cell| Element::Tag {
                        name: "td".into(),
                        attributes: Vec::new(),
                        children: vec![cell],
                    })
                    .collect::<Vec<_>>();
                Element::Tag {
                    name: "tr".into(),
                    attributes: Vec::new(),
                    children: cells,
                }
            })
            .collect::<Vec<_>>();

        if !body_rows.is_empty() {
            rows.push(Element::Tag {
                name: "tbody".into(),
                attributes: Vec::new(),
                children: body_rows,
            });
        }

        self.elements.push(Element::Tag {
            name: "table".into(),
            attributes: Vec::new(),
            children: rows,
        });
        self
    }

    #[must_use]
    pub fn compose(self, f: impl FnOnce(Body) -> Body) -> Self {
        f(self)
    }
}

impl Default for Body {
    fn default() -> Self {
        Self::new()
    }
}
