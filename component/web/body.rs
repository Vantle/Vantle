use element::{Element, Location};
use extraction::Extraction;
use language::Language;
use list::List;
use observe::trace;
use reference::Reference;
use span::Span;
use table::Table;

pub struct Body {
    pub elements: Vec<Element>,
    depth: u8,
}

impl Body {
    #[must_use]
    pub fn new() -> Self {
        Self {
            elements: Vec::new(),
            depth: 1,
        }
    }

    #[must_use]
    pub fn tag(mut self, name: &'static str, f: impl FnOnce(Body) -> Body) -> Self {
        let mut child = Body::new();
        child.depth = self.depth;
        let inner = f(child);
        self.elements.push(Element::Tag {
            name: name.into(),
            attributes: Vec::new(),
            children: inner.elements,
        });
        self
    }

    #[must_use]
    pub fn anchor(self, href: &str, f: impl FnOnce(Body) -> Body) -> Self {
        self.tag("a", f).attribute("href", href)
    }

    #[must_use]
    pub fn division(self, f: impl FnOnce(Body) -> Body) -> Self {
        self.tag("div", f)
    }

    #[must_use]
    pub fn image(mut self, source: &str, alternate: &str) -> Self {
        self.elements.push(Element::Void {
            name: "img".into(),
            attributes: vec![
                ("src".into(), source.into()),
                ("alt".into(), alternate.into()),
            ],
        });
        self
    }

    #[must_use]
    pub fn separator(mut self) -> Self {
        self.elements.push(Element::Void {
            name: "hr".into(),
            attributes: Vec::new(),
        });
        self
    }

    #[must_use]
    pub fn linebreak(mut self) -> Self {
        self.elements.push(Element::Void {
            name: "br".into(),
            attributes: Vec::new(),
        });
        self
    }

    #[must_use]
    pub fn preformatted(self, f: impl FnOnce(Body) -> Body) -> Self {
        self.tag("pre", f)
    }

    #[must_use]
    pub fn strong(self, f: impl FnOnce(Body) -> Body) -> Self {
        self.tag("strong", f)
    }

    #[must_use]
    pub fn unordered(self, f: impl FnOnce(Body) -> Body) -> Self {
        self.tag("ul", f)
    }

    #[must_use]
    pub fn ordered(self, f: impl FnOnce(Body) -> Body) -> Self {
        self.tag("ol", f)
    }

    #[must_use]
    pub fn item(self, f: impl FnOnce(Body) -> Body) -> Self {
        self.tag("li", f)
    }

    #[must_use]
    pub fn list(self, f: impl FnOnce(List) -> List) -> Self {
        self.items("ul", f)
    }

    #[must_use]
    pub fn enumeration(self, f: impl FnOnce(List) -> List) -> Self {
        self.items("ol", f)
    }

    fn items(mut self, name: &'static str, f: impl FnOnce(List) -> List) -> Self {
        let list = f(List::new());
        self.elements.push(Element::Tag {
            name: name.into(),
            attributes: Vec::new(),
            children: list.items,
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
        self.elements.extend(span.elements);
        self
    }

    #[must_use]
    pub fn html(mut self, raw: &str) -> Self {
        self.elements.push(Element::Raw(raw.into()));
        self
    }

    #[must_use]
    pub fn class(mut self, reference: Reference) -> Self {
        if let Some(Element::Tag { attributes, .. } | Element::Void { attributes, .. }) =
            self.elements.last_mut()
        {
            element::merge(attributes, "class", reference.name());
        }
        self
    }

    #[must_use]
    pub fn attribute(mut self, name: &str, value: &str) -> Self {
        if let Some(Element::Tag { attributes, .. } | Element::Void { attributes, .. }) =
            self.elements.last_mut()
        {
            attributes.push((name.into(), value.into()));
        }
        self
    }

    #[must_use]
    pub fn when(self, condition: bool, f: impl FnOnce(Self) -> Self) -> Self {
        if condition { f(self) } else { self }
    }

    #[must_use]
    pub fn code(mut self, content: &str, language: Language) -> Self {
        self.elements.push(Element::Code {
            content: content.into(),
            language,
            location: None,
        });
        self
    }

    #[must_use]
    pub fn extract(mut self, extraction: &Extraction) -> Self {
        self.elements.push(Element::Code {
            content: extraction.content.into(),
            language: extraction.language,
            location: Some(Location {
                source: extraction.name.into(),
                start: extraction.start,
                end: extraction.end,
            }),
        });
        self
    }

    #[must_use]
    pub fn shell(self, command: &str) -> Self {
        self.code(command, Language::Bash)
    }

    #[must_use]
    pub fn markdown(mut self, elements: Vec<element::Element>) -> Self {
        self.elements.extend(elements);
        self
    }

    #[must_use]
    pub fn table(self, f: impl FnOnce(Table) -> Table) -> Self {
        let table = f(Table::new());
        self.tag("table", |t| {
            let t = if table.headers.is_empty() {
                t
            } else {
                t.tag("thead", |h| {
                    h.tag("tr", |r| {
                        table
                            .headers
                            .into_iter()
                            .fold(r, |r, header| r.tag("th", |c| c.text(&header)))
                    })
                })
            };
            if table.rows.is_empty() {
                t
            } else {
                t.tag("tbody", |b| {
                    table.rows.into_iter().fold(b, |b, row| {
                        b.tag("tr", |r| {
                            row.into_iter().fold(r, |mut r, cell| {
                                r.elements.push(element("td", cell));
                                r
                            })
                        })
                    })
                })
            }
        })
    }

    #[must_use]
    pub fn compose(self, f: impl FnOnce(Body) -> Body) -> Self {
        f(self)
    }

    #[trace(channels = [document])]
    #[must_use]
    pub fn heading(self, level: u8, text: &str) -> Self {
        self.tag(element::HEADINGS[(level.clamp(1, 6) - 1) as usize], |e| {
            e.text(text)
        })
    }

    #[trace(channels = [document])]
    #[must_use]
    pub fn figure(self, source: &str, alternate: &str) -> Self {
        self.division(|d| d.image(source, alternate))
            .class(reference::center())
    }

    #[trace(channels = [document])]
    #[must_use]
    pub fn title(self, text: &str) -> Self {
        self.heading(1, text)
    }

    #[trace(channels = [document])]
    #[must_use]
    pub fn subtitle(self, text: &str) -> Self {
        self.tag("p", |p| p.text(text)).class(reference::subtitle())
    }

    #[trace(channels = [document])]
    #[must_use]
    pub fn rule(self) -> Self {
        self.separator()
    }

    #[trace(channels = [document])]
    #[must_use]
    pub fn paragraph(self, f: impl FnOnce(Span) -> Span) -> Self {
        self.tag("p", |p| p.span(f))
    }

    #[trace(channels = [document])]
    #[must_use]
    pub fn section(self, heading: &str, f: impl FnOnce(Body) -> Body) -> Self {
        let level = self.depth + 1;
        self.tag("section", |mut s| {
            s.depth = level;
            f(s.heading(level.min(6), heading))
        })
    }

    #[trace(channels = [document])]
    #[must_use]
    pub fn term(self, word: &str, f: impl FnOnce(Span) -> Span) -> Self {
        self.tag("dl", |dl| {
            dl.tag("dt", |dt| dt.text(word)).tag("dd", |dd| dd.span(f))
        })
    }

    #[trace(channels = [document])]
    #[must_use]
    pub fn aside(self, f: impl FnOnce(Span) -> Span) -> Self {
        self.tag("blockquote", |bq| bq.tag("p", |p| p.span(f)))
    }

    #[trace(channels = [document])]
    #[must_use]
    pub fn navigation(self, f: impl FnOnce(Body) -> Body) -> Self {
        self.tag("nav", f)
    }

    #[trace(channels = [document])]
    #[must_use]
    pub fn bold(self, text: &str) -> Self {
        self.strong(|e| e.text(text))
    }

    #[trace(channels = [document])]
    #[must_use]
    pub fn link(self, href: &str, text: &str) -> Self {
        self.anchor(href, |e| e.text(text))
    }

    #[trace(channels = [document])]
    #[must_use]
    pub fn definition(self, f: impl FnOnce(Body) -> Body) -> Self {
        self.tag("dl", f)
    }
}

fn element(name: &'static str, child: Element) -> Element {
    Element::Tag {
        name: name.into(),
        attributes: Vec::new(),
        children: vec![child],
    }
}

impl Default for Body {
    fn default() -> Self {
        Self::new()
    }
}
