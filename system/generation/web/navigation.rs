use body::{Body, Chain};
use observe::trace;
use page::Page;
use span::Span;

type Result = body::Result;

#[trace(channels = [document])]
pub fn layout(
    title: &str,
    index: &entry::Index,
    root: &str,
    f: impl FnOnce(Body) -> Result,
) -> page::Result {
    let context = index.context.as_str();
    let identifier = index.identifier;
    let favicon = format!("{root}resource/favicon.ico");
    Page::new()
            .title(&match (context, title) {
                ("vantle", "Vantle") | ("molten", "Molten") => title.to_string(),
                ("molten", _) => format!("Molten.{title}"),
                _ => format!("Vantle.{title}"),
            })
            .favicon(&favicon)
            .stylesheet(&format!("{root}resource/system/document/vantle.css"))
            .wasm(&format!("{root}resource/system/document/compute.js"))
            .context(context)
            .identifier(identifier)
            .root(root)
            .body(|b| {
                let molten = index.context == entry::Context::Molten;
                b.tag("nav", |n| {
                    let href = if molten {
                        index::molten::readme(root).href
                    } else {
                        index::readme(root).href
                    };
                    let source = if molten {
                        format!("{root}Molten/resource/logo.png")
                    } else {
                        format!("{root}resource/logo.png")
                    };
                    let alternate = if molten { "Molten" } else { "Vantle" };
                    n.anchor(&href, |a| {
                        a.class(class::nav::logo())
                            .image(&source, alternate)
                    })
                    .division(|d| {
                        d.class(class::nav::links())
                            .division(|dd| {
                                dd.class(class::nav::dropdown())
                                    .anchor(&index::readme(root).href, |a| {
                                        a.text("Vantle")
                                    })
                                    .division(|m| {
                                        m.class(class::nav::menu())
                                            .anchor(&index::info(root).href, |a| {
                                                a.text("Info")
                                            })
                                            .anchor(&index::notice(root).href, |a| {
                                                a.text("Notice")
                                            })
                                            .anchor(&index::module(root).href, |a| {
                                                a.text("Module")
                                            })
                                            .anchor(&index::license(root).href, |a| {
                                                a.text("License")
                                            })
                                    })
                            })
                            .division(|dd| {
                                dd.class(class::nav::dropdown())
                                    .anchor(&index::molten::readme(root).href, |a| {
                                        a.text("Molten")
                                    })
                                    .division(|m| {
                                        m.class(class::nav::menu())
                                            .anchor(
                                                &index::molten::spatialize::spatialize(root).href,
                                                |a| a.text("Spatialize"),
                                            )
                                            .separator()
                                            .anchor(
                                                &index::molten::info(root).href,
                                                |a| a.text("Info"),
                                            )
                                            .anchor(
                                                &index::molten::notice(root).href,
                                                |a| a.text("Notice"),
                                            )
                                            .anchor(
                                                &index::molten::license(root).href,
                                                |a| a.text("License"),
                                            )
                                    })
                            })
                            .division(|dd| {
                                dd.class(class::nav::dropdown())
                                    .anchor(
                                        &index::generation::generation(root).href,
                                        |a| a.text("Generation"),
                                    )
                                    .division(|m| {
                                        m.class(class::nav::menu())
                                            .anchor(
                                                &index::generation::web::web(root).href,
                                                |a| a.text("Web"),
                                            )
                                            .anchor(
                                                &index::generation::autotest::autotest(root).href,
                                                |a| a.text("Autotest"),
                                            )
                                            .anchor(
                                                &index::generation::autotest::function(root).href,
                                                |a| {
                                                    a.class(class::nav::nested())
                                                        .text("Function")
                                                },
                                            )
                                            .anchor(
                                                &index::generation::autotest::performance(root).href,
                                                |a| {
                                                    a.class(class::nav::nested())
                                                        .text("Performance")
                                                },
                                            )
                                            .anchor(
                                                &index::generation::extract(root).href,
                                                |a| a.text("Extract"),
                                            )
                                    })
                            })
                            .anchor(&index::observation::observation(root).href, |a| {
                                a.text("Observation")
                            })
                            .anchor(&index::spatialize::spatialize(root).href, |a| {
                                a.text("Spatialize")
                            })
                    })
                })
                .division(|l| {
                    l.class(class::layout())
                        .tag("aside", |a| sidebar(a, root, index))
                        .tag("main", |m| {
                            f(m)
                                .tag("footer", |footer| {
                                    footer.tag("p", |p| {
                                        p.span(|s| {
                                            s.text("\u{00a9} 2025-2026 Vantle \u{00b7} ")
                                                .link("https://vantle.org", "@robert.vanderzee")
                                        })
                                    })
                                    .anchor("https://github.com/Vantle/Vantle", |a| {
                                        a.class(class::footer::icon())
                                            .attribute("aria-label", "GitHub")
                                            .html("<svg width=\"16\" height=\"16\" viewBox=\"0 0 16 16\" fill=\"currentColor\"><path d=\"M8 0C3.58 0 0 3.58 0 8c0 3.54 2.29 6.53 5.47 7.59.4.07.55-.17.55-.38 0-.19-.01-.82-.01-1.49-2.01.37-2.53-.49-2.69-.94-.09-.23-.48-.94-.82-1.13-.28-.15-.68-.52-.01-.53.63-.01 1.08.58 1.23.82.72 1.21 1.87.87 2.33.66.07-.52.28-.87.51-1.07-1.78-.2-3.64-.89-3.64-3.95 0-.87.31-1.59.82-2.15-.08-.2-.36-1.02.08-2.12 0 0 .67-.21 2.2.82.64-.18 1.32-.27 2-.27.68 0 1.36.09 2 .27 1.53-1.04 2.2-.82 2.2-.82.44 1.1.16 1.92.08 2.12.51.56.82 1.27.82 2.15 0 3.07-1.87 3.75-3.65 3.95.29.25.54.73.54 1.48 0 1.07-.01 1.93-.01 2.2 0 .21.15.46.55.38A8.013 8.013 0 0016 8c0-4.42-3.58-8-8-8z\"/></svg>")
                                    })
                                })
                        })
                        .tag("aside", |a| {
                            a.class(class::outline())
                                .attribute("aria-label", "Table of contents")
                        })
                })
            })
}

fn sidebar(body: Body, root: &str, current: &entry::Index) -> Result {
    type Links = Vec<(&'static str, entry::Index, bool)>;
    let (primary, legal): (Links, Links) = match current.context {
        entry::Context::Molten => (
            vec![("Molten", index::molten::readme(root), false)],
            vec![
                ("Info", index::molten::info(root), false),
                ("Notice", index::molten::notice(root), false),
                ("License", index::molten::license(root), false),
            ],
        ),
        entry::Context::Generation => (
            vec![
                ("Generation", index::generation::generation(root), false),
                ("Web", index::generation::web::web(root), false),
                (
                    "Autotest",
                    index::generation::autotest::autotest(root),
                    false,
                ),
                ("Extract", index::generation::extract(root), false),
            ],
            vec![
                ("Info", index::info(root), false),
                ("Notice", index::notice(root), false),
                ("License", index::license(root), false),
            ],
        ),
        entry::Context::Autotest => (
            vec![
                (
                    "Autotest",
                    index::generation::autotest::autotest(root),
                    false,
                ),
                (
                    "Function",
                    index::generation::autotest::function(root),
                    true,
                ),
                (
                    "Performance",
                    index::generation::autotest::performance(root),
                    true,
                ),
            ],
            vec![
                ("Info", index::info(root), false),
                ("Notice", index::notice(root), false),
                ("License", index::license(root), false),
            ],
        ),
        entry::Context::Vantle => (
            vec![
                ("Vantle", index::readme(root), false),
                ("Module", index::module(root), false),
            ],
            vec![
                ("Info", index::info(root), false),
                ("Notice", index::notice(root), false),
                ("License", index::license(root), false),
            ],
        ),
    };

    let result = body
        .class(class::sidebar())
        .attribute("aria-label", "Page navigation")
        .division(|d| d.class(class::label::sidebar()).text("Pages"));

    let result = primary
        .into_iter()
        .fold(result, |b, (text, entry, nested)| {
            b.anchor(&entry.href, |a| {
                let a = if nested {
                    a.class(class::nav::nested())
                } else {
                    Ok(a)
                };
                let a = a.text(text);
                if entry.identifier == current.identifier {
                    a.attribute("aria-current", "page")
                } else {
                    a
                }
            })
        });

    if legal.is_empty() {
        return result;
    }

    let result = result.division(|d| d.class(class::label::sidebar()).text("Legal"));

    legal.into_iter().fold(result, |b, (text, entry, _)| {
        b.anchor(&entry.href, |a| {
            let a = a.text(text);
            if entry.identifier == current.identifier {
                a.attribute("aria-current", "page")
            } else {
                a
            }
        })
    })
}

pub trait Composition {
    fn heading(self, level: u8, text: &str) -> Result;
    fn figure(self, source: &str, alternate: &str) -> Result;
    fn title(self, text: &str) -> Result;
    fn subtitle(self, text: &str) -> Result;
    fn rule(self) -> Result;
    fn paragraph(self, f: impl FnOnce(Span) -> Span) -> Result;
    fn section(self, title: &str, f: impl FnOnce(Body) -> Result) -> Result;
    fn subsection(self, title: &str, f: impl FnOnce(Body) -> Result) -> Result;
    fn term(self, word: &str, f: impl FnOnce(Span) -> Span) -> Result;
    fn aside(self, f: impl FnOnce(Span) -> Span) -> Result;
    fn navigation(self, f: impl FnOnce(Body) -> Result) -> Result;
    fn bold(self, text: &str) -> Result;
    fn link(self, href: &str, text: &str) -> Result;
    fn definition(self, f: impl FnOnce(Body) -> Result) -> Result;
}

impl Composition for Body {
    #[trace(channels = [document])]
    fn heading(self, level: u8, text: &str) -> Result {
        self.tag(&format!("h{}", level.clamp(1, 6)), |e| e.text(text))
    }

    #[trace(channels = [document])]
    fn figure(self, source: &str, alternate: &str) -> Result {
        self.division(|d| d.class(class::center()).image(source, alternate))
    }

    #[trace(channels = [document])]
    fn title(self, text: &str) -> Result {
        self.heading(1, text)
    }

    #[trace(channels = [document])]
    fn subtitle(self, text: &str) -> Result {
        self.tag("p", |p| p.class(class::subtitle()).text(text))
    }

    #[trace(channels = [document])]
    fn rule(self) -> Result {
        self.separator()
    }

    #[trace(channels = [document])]
    fn paragraph(self, f: impl FnOnce(Span) -> Span) -> Result {
        self.tag("p", |p| p.span(f))
    }

    #[trace(channels = [document])]
    fn section(self, title: &str, f: impl FnOnce(Body) -> Result) -> Result {
        self.tag("section", |s| f(s.heading(2, title)?))
    }

    #[trace(channels = [document])]
    fn subsection(self, title: &str, f: impl FnOnce(Body) -> Result) -> Result {
        self.tag("section", |s| f(s.heading(3, title)?))
    }

    #[trace(channels = [document])]
    fn term(self, word: &str, f: impl FnOnce(Span) -> Span) -> Result {
        self.tag("dl", |dl| {
            dl.tag("dt", |dt| dt.text(word)).tag("dd", |dd| dd.span(f))
        })
    }

    #[trace(channels = [document])]
    fn aside(self, f: impl FnOnce(Span) -> Span) -> Result {
        self.tag("blockquote", |bq| bq.tag("p", |p| p.span(f)))
    }

    #[trace(channels = [document])]
    fn navigation(self, f: impl FnOnce(Body) -> Result) -> Result {
        self.tag("nav", f)
    }

    #[trace(channels = [document])]
    fn bold(self, text: &str) -> Result {
        self.strong(|e| e.text(text))
    }

    #[trace(channels = [document])]
    fn link(self, href: &str, text: &str) -> Result {
        self.anchor(href, |e| e.text(text))
    }

    #[trace(channels = [document])]
    fn definition(self, f: impl FnOnce(Body) -> Result) -> Result {
        self.tag("dl", f)
    }
}

impl Composition for Result {
    fn heading(self, level: u8, text: &str) -> Result {
        self?.heading(level, text)
    }

    fn figure(self, source: &str, alternate: &str) -> Result {
        self?.figure(source, alternate)
    }

    fn title(self, text: &str) -> Result {
        self?.title(text)
    }

    fn subtitle(self, text: &str) -> Result {
        self?.subtitle(text)
    }

    fn rule(self) -> Result {
        self?.rule()
    }

    fn paragraph(self, f: impl FnOnce(Span) -> Span) -> Result {
        self?.paragraph(f)
    }

    fn section(self, title: &str, f: impl FnOnce(Body) -> Result) -> Result {
        self?.section(title, f)
    }

    fn subsection(self, title: &str, f: impl FnOnce(Body) -> Result) -> Result {
        self?.subsection(title, f)
    }

    fn term(self, word: &str, f: impl FnOnce(Span) -> Span) -> Result {
        self?.term(word, f)
    }

    fn aside(self, f: impl FnOnce(Span) -> Span) -> Result {
        self?.aside(f)
    }

    fn navigation(self, f: impl FnOnce(Body) -> Result) -> Result {
        self?.navigation(f)
    }

    fn bold(self, text: &str) -> Result {
        self?.bold(text)
    }

    fn link(self, href: &str, text: &str) -> Result {
        self?.link(href, text)
    }

    fn definition(self, f: impl FnOnce(Body) -> Result) -> Result {
        self?.definition(f)
    }
}
