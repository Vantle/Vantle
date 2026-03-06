use class::Reference;
use element::{Element, Location};
use extraction::Extraction;
use language::Language;
use list::List;
use span::Span;
use table::Table;

pub type Result = miette::Result<Body>;

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

    pub fn tag(mut self, name: &str, f: impl FnOnce(Body) -> Result) -> Result {
        let inner = f(Body::new())?;
        self.elements.push(Element::Tag {
            name: name.into(),
            attributes: inner.pending,
            children: inner.elements,
        });
        Ok(self)
    }

    pub fn anchor(self, href: &str, f: impl FnOnce(Body) -> Result) -> Result {
        self.tag("a", |a| f(a.attribute("href", href)?))
    }

    pub fn division(self, f: impl FnOnce(Body) -> Result) -> Result {
        self.tag("div", f)
    }

    pub fn image(self, source: &str, alternate: &str) -> Result {
        self.tag("img", |i| {
            i.attribute("src", source).attribute("alt", alternate)
        })
    }

    pub fn separator(self) -> Result {
        self.tag("hr", Ok)
    }

    pub fn linebreak(self) -> Result {
        self.tag("br", Ok)
    }

    pub fn preformatted(self, f: impl FnOnce(Body) -> Result) -> Result {
        self.tag("pre", f)
    }

    pub fn strong(self, f: impl FnOnce(Body) -> Result) -> Result {
        self.tag("strong", f)
    }

    pub fn unordered(self, f: impl FnOnce(Body) -> Result) -> Result {
        self.tag("ul", f)
    }

    pub fn ordered(self, f: impl FnOnce(Body) -> Result) -> Result {
        self.tag("ol", f)
    }

    pub fn item(self, f: impl FnOnce(Body) -> Result) -> Result {
        self.tag("li", f)
    }

    pub fn list(mut self, f: impl FnOnce(List) -> List) -> Result {
        let list = f(List::new());
        self.elements.push(Element::Tag {
            name: "ul".into(),
            attributes: Vec::new(),
            children: list.items,
        });
        Ok(self)
    }

    pub fn enumeration(mut self, f: impl FnOnce(List) -> List) -> Result {
        let list = f(List::new());
        self.elements.push(Element::Tag {
            name: "ol".into(),
            attributes: Vec::new(),
            children: list.items,
        });
        Ok(self)
    }

    pub fn text(mut self, content: &str) -> Result {
        self.elements.push(Element::Text(content.into()));
        Ok(self)
    }

    pub fn span(mut self, f: impl FnOnce(Span) -> Span) -> Result {
        let span = f(Span::new());
        self.elements.push(Element::Span(span.fragments));
        Ok(self)
    }

    pub fn html(mut self, raw: &str) -> Result {
        self.elements.push(Element::Raw(raw.into()));
        Ok(self)
    }

    pub fn class(mut self, reference: Reference) -> Result {
        let name = reference.name();
        if let Some(existing) = self.pending.iter_mut().find(|(k, _)| k == "class") {
            existing.1.push(' ');
            existing.1.push_str(name);
        } else {
            self.pending.push(("class".into(), name.into()));
        }
        Ok(self)
    }

    pub fn attribute(mut self, name: &str, value: &str) -> Result {
        self.pending.push((name.into(), value.into()));
        Ok(self)
    }

    pub fn code(mut self, content: &str, language: Language) -> Result {
        self.elements.push(Element::Code {
            content: content.into(),
            language,
            location: None,
        });
        Ok(self)
    }

    pub fn extract(mut self, extraction: &Extraction) -> Result {
        self.elements.push(Element::Code {
            content: extraction.content.into(),
            language: extraction.language,
            location: Some(Location {
                source: extraction.name.into(),
                start: extraction.start,
                end: extraction.end,
            }),
        });
        Ok(self)
    }

    pub fn shell(mut self, command: &str) -> Result {
        self.elements.push(Element::Shell {
            command: command.into(),
        });
        Ok(self)
    }

    pub fn inject(mut self, name: &str) -> Result {
        self.elements.push(Element::Inject { name: name.into() });
        Ok(self)
    }

    pub fn markdown(mut self, name: &str) -> Result {
        self.elements.push(Element::Markdown { name: name.into() });
        Ok(self)
    }

    pub fn table(mut self, f: impl FnOnce(Table) -> Table) -> Result {
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
        Ok(self)
    }

    pub fn compose(self, f: impl FnOnce(Body) -> Result) -> Result {
        f(self)
    }
}

impl Default for Body {
    fn default() -> Self {
        Self::new()
    }
}

pub trait Chain {
    fn tag(self, name: &str, f: impl FnOnce(Body) -> Result) -> Result;
    fn anchor(self, href: &str, f: impl FnOnce(Body) -> Result) -> Result;
    fn division(self, f: impl FnOnce(Body) -> Result) -> Result;
    fn image(self, source: &str, alternate: &str) -> Result;
    fn separator(self) -> Result;
    fn linebreak(self) -> Result;
    fn preformatted(self, f: impl FnOnce(Body) -> Result) -> Result;
    fn strong(self, f: impl FnOnce(Body) -> Result) -> Result;
    fn unordered(self, f: impl FnOnce(Body) -> Result) -> Result;
    fn ordered(self, f: impl FnOnce(Body) -> Result) -> Result;
    fn item(self, f: impl FnOnce(Body) -> Result) -> Result;
    fn list(self, f: impl FnOnce(List) -> List) -> Result;
    fn enumeration(self, f: impl FnOnce(List) -> List) -> Result;
    fn text(self, content: &str) -> Result;
    fn span(self, f: impl FnOnce(Span) -> Span) -> Result;
    fn html(self, raw: &str) -> Result;
    fn class(self, reference: Reference) -> Result;
    fn attribute(self, name: &str, value: &str) -> Result;
    fn code(self, content: &str, language: Language) -> Result;
    fn extract(self, extraction: &Extraction) -> Result;
    fn shell(self, command: &str) -> Result;
    fn inject(self, name: &str) -> Result;
    fn markdown(self, name: &str) -> Result;
    fn table(self, f: impl FnOnce(Table) -> Table) -> Result;
    fn compose(self, f: impl FnOnce(Body) -> Result) -> Result;
}

impl Chain for Result {
    fn tag(self, name: &str, f: impl FnOnce(Body) -> Result) -> Result {
        self?.tag(name, f)
    }

    fn anchor(self, href: &str, f: impl FnOnce(Body) -> Result) -> Result {
        self?.anchor(href, f)
    }

    fn division(self, f: impl FnOnce(Body) -> Result) -> Result {
        self?.division(f)
    }

    fn image(self, source: &str, alternate: &str) -> Result {
        self?.image(source, alternate)
    }

    fn separator(self) -> Result {
        self?.separator()
    }

    fn linebreak(self) -> Result {
        self?.linebreak()
    }

    fn preformatted(self, f: impl FnOnce(Body) -> Result) -> Result {
        self?.preformatted(f)
    }

    fn strong(self, f: impl FnOnce(Body) -> Result) -> Result {
        self?.strong(f)
    }

    fn unordered(self, f: impl FnOnce(Body) -> Result) -> Result {
        self?.unordered(f)
    }

    fn ordered(self, f: impl FnOnce(Body) -> Result) -> Result {
        self?.ordered(f)
    }

    fn item(self, f: impl FnOnce(Body) -> Result) -> Result {
        self?.item(f)
    }

    fn list(self, f: impl FnOnce(List) -> List) -> Result {
        self?.list(f)
    }

    fn enumeration(self, f: impl FnOnce(List) -> List) -> Result {
        self?.enumeration(f)
    }

    fn text(self, content: &str) -> Result {
        self?.text(content)
    }

    fn span(self, f: impl FnOnce(Span) -> Span) -> Result {
        self?.span(f)
    }

    fn html(self, raw: &str) -> Result {
        self?.html(raw)
    }

    fn class(self, reference: Reference) -> Result {
        self?.class(reference)
    }

    fn attribute(self, name: &str, value: &str) -> Result {
        self?.attribute(name, value)
    }

    fn code(self, content: &str, language: Language) -> Result {
        self?.code(content, language)
    }

    fn extract(self, extraction: &Extraction) -> Result {
        self?.extract(extraction)
    }

    fn shell(self, command: &str) -> Result {
        self?.shell(command)
    }

    fn inject(self, name: &str) -> Result {
        self?.inject(name)
    }

    fn markdown(self, name: &str) -> Result {
        self?.markdown(name)
    }

    fn table(self, f: impl FnOnce(Table) -> Table) -> Result {
        self?.table(f)
    }

    fn compose(self, f: impl FnOnce(Body) -> Result) -> Result {
        self?.compose(f)
    }
}
