use body::Body;
use observe::trace;
use page::Page;

#[trace(channels = [document])]
pub fn layout(title: &str, index: &entry::Index, root: &str, f: impl FnOnce(Body) -> Body) -> Page {
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
                    n.anchor(&href, |a| a.image(&source, alternate))
                        .class(class::nav::logo())
                        .division(|d| {
                            d.division(|dd| {
                                dd.link(&index::readme(root).href, "Vantle")
                                    .division(|m| {
                                        m.link(&index::info(root).href, "Info")
                                            .link(&index::notice(root).href, "Notice")
                                            .link(&index::module(root).href, "Module")
                                            .link(&index::license(root).href, "License")
                                    })
                                    .class(class::nav::menu())
                            })
                            .class(class::nav::dropdown())
                            .division(|dd| {
                                dd.link(&index::molten::readme(root).href, "Molten")
                                    .division(|m| {
                                        m.link(
                                            &index::molten::spatialize::spatialize(root).href,
                                            "Spatialize",
                                        )
                                        .separator()
                                        .link(&index::molten::info(root).href, "Info")
                                        .link(&index::molten::notice(root).href, "Notice")
                                        .link(&index::molten::license(root).href, "License")
                                    })
                                    .class(class::nav::menu())
                            })
                            .class(class::nav::dropdown())
                            .division(|dd| {
                                dd.link(
                                    &index::generation::generation(root).href,
                                    "Generation",
                                )
                                .division(|m| {
                                    m.link(
                                        &index::generation::web::web(root).href,
                                        "Web",
                                    )
                                    .link(
                                        &index::generation::autotest::autotest(root).href,
                                        "Autotest",
                                    )
                                    .link(
                                        &index::generation::autotest::function(root).href,
                                        "Function",
                                    )
                                    .class(class::nav::nested())
                                    .link(
                                        &index::generation::autotest::performance(root).href,
                                        "Performance",
                                    )
                                    .class(class::nav::nested())
                                    .link(
                                        &index::generation::extract(root).href,
                                        "Extract",
                                    )
                                })
                                .class(class::nav::menu())
                            })
                            .class(class::nav::dropdown())
                            .link(&index::observation::observation(root).href, "Observation")
                            .link(&index::spatialize::spatialize(root).href, "Spatialize")
                        })
                        .class(class::nav::links())
                })
                .division(|l| {
                    l.tag("aside", |a| sidebar(a, root, index))
                        .class(class::sidebar())
                        .attribute("aria-label", "Page navigation")
                        .tag("main", |m| {
                            f(m)
                                .tag("footer", |footer| {
                                    footer
                                        .tag("p", |p| {
                                            p.span(|s| {
                                                s.text("\u{00a9} 2025-2026 Vantle \u{00b7} ")
                                                    .link(
                                                        "https://vantle.org",
                                                        |l| l.text("@robert.vanderzee"),
                                                    )
                                            })
                                        })
                                        .anchor(
                                            "https://github.com/Vantle/Vantle",
                                            |a| {
                                                a.html("<svg width=\"16\" height=\"16\" viewBox=\"0 0 16 16\" fill=\"currentColor\"><path d=\"M8 0C3.58 0 0 3.58 0 8c0 3.54 2.29 6.53 5.47 7.59.4.07.55-.17.55-.38 0-.19-.01-.82-.01-1.49-2.01.37-2.53-.49-2.69-.94-.09-.23-.48-.94-.82-1.13-.28-.15-.68-.52-.01-.53.63-.01 1.08.58 1.23.82.72 1.21 1.87.87 2.33.66.07-.52.28-.87.51-1.07-1.78-.2-3.64-.89-3.64-3.95 0-.87.31-1.59.82-2.15-.08-.2-.36-1.02.08-2.12 0 0 .67-.21 2.2.82.64-.18 1.32-.27 2-.27.68 0 1.36.09 2 .27 1.53-1.04 2.2-.82 2.2-.82.44 1.1.16 1.92.08 2.12.51.56.82 1.27.82 2.15 0 3.07-1.87 3.75-3.65 3.95.29.25.54.73.54 1.48 0 1.07-.01 1.93-.01 2.2 0 .21.15.46.55.38A8.013 8.013 0 0016 8c0-4.42-3.58-8-8-8z\"/></svg>")
                                            },
                                        )
                                        .class(class::footer::icon())
                                        .attribute("aria-label", "GitHub")
                                })
                        })
                        .tag("aside", |b| b)
                        .attribute("id", "outline")
                        .class(class::outline())
                        .attribute("aria-label", "Table of contents")
                })
                .class(class::layout())
            })
}

fn sidebar(body: Body, root: &str, current: &entry::Index) -> Body {
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
        .division(|d| d.text("Pages"))
        .class(class::label::sidebar());

    let result = primary
        .into_iter()
        .fold(result, |b, (text, entry, nested)| {
            b.link(&entry.href, text)
                .when(nested, |b| b.class(class::nav::nested()))
                .when(entry.identifier == current.identifier, |b| {
                    b.attribute("aria-current", "page")
                })
        });

    if legal.is_empty() {
        return result;
    }

    let result = result
        .division(|d| d.text("Legal"))
        .class(class::label::sidebar());

    legal.into_iter().fold(result, |b, (text, entry, _)| {
        b.link(&entry.href, text)
            .when(entry.identifier == current.identifier, |b| {
                b.attribute("aria-current", "page")
            })
    })
}
