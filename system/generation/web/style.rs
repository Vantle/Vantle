use body::Body;
use element::Language;
use observe::trace;
use page::Page;
use span::Span;
use style::Style;

#[trace(channels = [document])]
fn scale(k: i32) -> String {
    format!("{}rem", proportion::scale(k))
}

fn half(k: i32) -> String {
    format!("{}rem", proportion::half(k))
}

fn grid() -> String {
    let side = proportion::scale(-3);
    format!("{side}fr 1fr {side}fr")
}

#[trace(channels = [document])]
fn variables(style: Style) -> Style {
    let style = palette::PALETTE
        .iter()
        .chain(palette::SYNTAX)
        .fold(style, |s, token| {
            s.variable(&format!("--{}", token.role), token.light)
        });
    style
        .variable("--phi", &proportion::PHI.to_string())
        .variable("--scale-n2", &scale(-2))
        .variable("--scale-n1", &scale(-1))
        .variable("--scale-n0h", &half(0))
        .variable("--scale-0", &scale(0))
        .variable("--scale-1", &scale(1))
        .variable("--scale-2", &scale(2))
        .variable("--scale-3", &scale(3))
}

#[trace(channels = [document])]
fn dark(properties: style::Properties) -> style::Properties {
    palette::PALETTE
        .iter()
        .chain(palette::SYNTAX)
        .fold(properties, |p, token| {
            p.custom(&format!("--{}", token.role), token.dark)
        })
}

#[trace(channels = [document])]
pub fn page(
    arguments: &html::Arguments,
    title: &str,
    context: &str,
    identifier: &str,
    f: impl FnOnce(Body) -> Body,
) -> miette::Result<()> {
    let root = arguments.root();
    let favicon = format!("{root}resource/favicon.ico");
    html::generate(
        arguments,
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
            .root(&root)
            .body(|b| {
                let molten = context == "molten";
                b.element("nav", |n| {
                    let logo_href = if molten {
                        format!("{root}Molten/")
                    } else {
                        format!("{root}index.html")
                    };
                    let logo_src = if molten {
                        format!("{root}Molten/resource/logo.png")
                    } else {
                        format!("{root}resource/logo.png")
                    };
                    let logo_alt = if molten { "Molten" } else { "Vantle" };
                    n.element("a", |a| {
                        a.class("nav-logo")
                            .attribute("href", &logo_href)
                            .element("img", |i| {
                                i.attribute("src", &logo_src).attribute("alt", logo_alt)
                            })
                    })
                    .element("div", |d| {
                        d.class("nav-links")
                            .element("div", |dd| {
                                dd.class("nav-dropdown")
                                    .element("a", |a| {
                                        a.attribute("href", &format!("{root}index.html"))
                                            .text("Vantle")
                                    })
                                    .element("div", |m| {
                                        m.class("nav-dropdown-menu")
                                            .element("a", |a| {
                                                a.attribute("href", &format!("{root}info.html"))
                                                    .text("Info")
                                            })
                                            .element("a", |a| {
                                                a.attribute("href", &format!("{root}notice.html"))
                                                    .text("Notice")
                                            })
                                            .element("a", |a| {
                                                a.attribute("href", &format!("{root}module.html"))
                                                    .text("Module")
                                            })
                                            .element("a", |a| {
                                                a.attribute("href", &format!("{root}license.html"))
                                                    .text("License")
                                            })
                                    })
                            })
                            .element("div", |dd| {
                                dd.class("nav-dropdown")
                                    .element("a", |a| {
                                        a.attribute("href", &format!("{root}Molten/"))
                                            .text("Molten")
                                    })
                                    .element("div", |m| {
                                        m.class("nav-dropdown-menu")
                                            .element("a", |a| {
                                                a.attribute(
                                                    "href",
                                                    &format!("{root}Molten/system/spatialize/"),
                                                )
                                                .text("Spatialize")
                                            })
                                            .element("hr", |h| h)
                                            .element("a", |a| {
                                                a.attribute(
                                                    "href",
                                                    &format!("{root}Molten/info.html"),
                                                )
                                                .text("Info")
                                            })
                                            .element("a", |a| {
                                                a.attribute(
                                                    "href",
                                                    &format!("{root}Molten/notice.html"),
                                                )
                                                .text("Notice")
                                            })
                                            .element("a", |a| {
                                                a.attribute(
                                                    "href",
                                                    &format!("{root}Molten/license.html"),
                                                )
                                                .text("License")
                                            })
                                    })
                            })
                            .element("div", |dd| {
                                dd.class("nav-dropdown")
                                    .element("a", |a| {
                                        a.attribute(
                                            "href",
                                            &format!("{root}system/generation/"),
                                        )
                                        .text("Generation")
                                    })
                                    .element("div", |m| {
                                        m.class("nav-dropdown-menu")
                                            .element("a", |a| {
                                                a.attribute(
                                                    "href",
                                                    &format!(
                                                        "{root}system/generation/web/"
                                                    ),
                                                )
                                                .text("Web")
                                            })
                                            .element("a", |a| {
                                                a.attribute(
                                                    "href",
                                                    &format!(
                                                        "{root}system/generation/rust/"
                                                    ),
                                                )
                                                .text("Autotest")
                                            })
                                    })
                            })
                            .element("a", |a| {
                                a.attribute("href", &format!("{root}system/observation/"))
                                    .text("Observation")
                            })
                            .element("a", |a| {
                                a.attribute("href", &format!("{root}system/spatialize/"))
                                    .text("Spatialize")
                            })
                    })
                })
                .element("div", |l| {
                    l.class("layout")
                        .element("aside", |a| sidebar(a, &root, context, identifier))
                        .element("main", |m| {
                            f(m).element("footer", |footer| {
                                footer.element("p", |p| {
                                    p.span(|s| {
                                        s.text("\u{00a9} 2025 Vantle \u{00b7} ")
                                            .link("https://vantle.org", "@robert.vanderzee")
                                    })
                                })
                                .element("a", |a| {
                                    a.class("footer-icon")
                                        .attribute("href", "https://github.com/Vantle/Vantle")
                                        .attribute("aria-label", "GitHub")
                                        .html("<svg width=\"16\" height=\"16\" viewBox=\"0 0 16 16\" fill=\"currentColor\"><path d=\"M8 0C3.58 0 0 3.58 0 8c0 3.54 2.29 6.53 5.47 7.59.4.07.55-.17.55-.38 0-.19-.01-.82-.01-1.49-2.01.37-2.53-.49-2.69-.94-.09-.23-.48-.94-.82-1.13-.28-.15-.68-.52-.01-.53.63-.01 1.08.58 1.23.82.72 1.21 1.87.87 2.33.66.07-.52.28-.87.51-1.07-1.78-.2-3.64-.89-3.64-3.95 0-.87.31-1.59.82-2.15-.08-.2-.36-1.02.08-2.12 0 0 .67-.21 2.2.82.64-.18 1.32-.27 2-.27.68 0 1.36.09 2 .27 1.53-1.04 2.2-.82 2.2-.82.44 1.1.16 1.92.08 2.12.51.56.82 1.27.82 2.15 0 3.07-1.87 3.75-3.65 3.95.29.25.54.73.54 1.48 0 1.07-.01 1.93-.01 2.2 0 .21.15.46.55.38A8.013 8.013 0 0016 8c0-4.42-3.58-8-8-8z\"/></svg>")
                                })
                            })
                        })
                        .element("aside", |a| {
                            a.class("outline")
                                .attribute("aria-label", "Table of contents")
                        })
                })
            }),
    )
}

fn sidebar(body: Body, root: &str, context: &str, identifier: &str) -> Body {
    type Links<'a> = Vec<(&'a str, String, &'a str)>;
    let (primary, legal): (Links<'_>, Links<'_>) = match context {
        "molten" => (
            vec![("Molten", format!("{root}Molten/"), "readme")],
            vec![
                ("Info", format!("{root}Molten/info.html"), "info"),
                ("Notice", format!("{root}Molten/notice.html"), "notice"),
                ("License", format!("{root}Molten/license.html"), "license"),
            ],
        ),
        "generation" => (
            vec![
                (
                    "Generation",
                    format!("{root}system/generation/"),
                    "generation",
                ),
                ("Web", format!("{root}system/generation/web/"), "web"),
                (
                    "Autotest",
                    format!("{root}system/generation/rust/"),
                    "autotest",
                ),
            ],
            vec![
                ("Info", format!("{root}info.html"), "info"),
                ("Notice", format!("{root}notice.html"), "notice"),
                ("License", format!("{root}license.html"), "license"),
            ],
        ),
        _ => (
            vec![
                ("Vantle", format!("{root}index.html"), "readme"),
                ("Module", format!("{root}module.html"), "module"),
            ],
            vec![
                ("Info", format!("{root}info.html"), "info"),
                ("Notice", format!("{root}notice.html"), "notice"),
                ("License", format!("{root}license.html"), "license"),
            ],
        ),
    };

    let body = body
        .class("sidebar")
        .attribute("aria-label", "Page navigation")
        .element("div", |d| d.class("sidebar-label").text("Pages"));

    let body = primary.into_iter().fold(body, |b, (text, href, id)| {
        b.element("a", |a| {
            let a = a.attribute("href", &href).text(text);
            if id == identifier {
                a.attribute("aria-current", "page")
            } else {
                a
            }
        })
    });

    if legal.is_empty() {
        return body;
    }

    let body = body.element("div", |d| d.class("sidebar-label").text("Legal"));

    legal.into_iter().fold(body, |b, (text, href, id)| {
        b.element("a", |a| {
            let a = a.attribute("href", &href).text(text);
            if id == identifier {
                a.attribute("aria-current", "page")
            } else {
                a
            }
        })
    })
}

#[trace(channels = [document])]
#[must_use]
pub fn theme() -> Style {
    variables(Style::new())
        .rule("html", |r| {
            r.custom(
                "scroll-padding-top",
                "calc(var(--scale-3) + var(--scale-n2) + var(--scale-n2))",
            )
        })
        .rule("*", |r| r.margin("0").padding("0").box_sizing("border-box"))
        .rule("body", |r| {
            r.font_family("-apple-system, BlinkMacSystemFont, 'Segoe UI', system-ui, sans-serif")
                .background("var(--background)")
                .color("var(--text)")
                .line_height(&proportion::PHI.to_string())
                .font_size("var(--scale-0)")
                .transition("background-color 0.3s ease, color 0.3s ease")
        })
        .rule("main", |r| {
            r.padding("var(--scale-2) var(--scale-1)")
                .custom("min-width", "0")
        })
        .rule("h1", |r| {
            r.font_size("var(--scale-3)")
                .font_weight("700")
                .margin_bottom("var(--scale-n1)")
                .line_height("1.1")
                .custom("letter-spacing", "-0.03em")
        })
        .rule("h2", |r| {
            r.font_size("var(--scale-2)")
                .font_weight("600")
                .margin_top("var(--scale-2)")
                .margin_bottom("var(--scale-0)")
                .custom("letter-spacing", "-0.02em")
        })
        .rule("h3", |r| {
            r.font_size("var(--scale-1)")
                .font_weight("600")
                .margin_top("var(--scale-1)")
                .margin_bottom("var(--scale-n1)")
                .custom("letter-spacing", "-0.01em")
        })
        .rule("h4", |r| {
            r.font_size("var(--scale-0)")
                .font_weight("600")
                .custom("letter-spacing", "-0.01em")
        })
        .rule("h5", |r| {
            r.font_size("var(--scale-0)")
                .font_weight("500")
                .color("var(--text-secondary)")
        })
        .rule("p", |r| r.margin_bottom("var(--scale-0)"))
        .rule("a", |r| {
            r.color("var(--accent)")
                .text_decoration("none")
                .transition("color 0.2s")
        })
        .rule("a:hover", |r| {
            r.color("var(--accent-hover)").text_decoration("underline")
        })
        .rule("code", |r| {
            r.font_family("'SF Mono', 'Fira Code', 'Cascadia Code', monospace")
                .font_size("var(--scale-0)")
                .background("var(--code-background)")
                .color("var(--code-text)")
                .padding("0.15em 0.4em")
                .border_radius("4px")
                .transition("background-color 0.3s ease, color 0.3s ease")
        })
        .rule("pre", |r| {
            r.font_family("'SF Mono', 'Fira Code', 'Cascadia Code', monospace")
                .font_size("var(--scale-0)")
                .background("var(--code-background)")
                .border_radius("6px")
                .padding("var(--scale-0)")
                .overflow("auto")
                .margin_bottom("var(--scale-0)")
                .line_height("1.5")
                .transition("background-color 0.3s ease")
        })
        .rule("pre code", |r| {
            r.background("transparent").padding("0").border_radius("0")
        })
        .rule(".code-block", |r| {
            r.font_family("'SF Mono', 'Fira Code', 'Cascadia Code', monospace")
                .font_size("var(--scale-0)")
                .background("var(--code-background)")
                .border_radius("6px")
                .padding("var(--scale-0)")
                .overflow("auto")
                .margin_bottom("var(--scale-0)")
                .line_height("1.5")
                .position("relative")
                .white_space("pre-wrap")
                .transition("background-color 0.3s ease")
        })
        .rule(".code-block .syntax-keyword", |r| {
            r.color("var(--syntax-keyword)")
        })
        .rule(".code-block .syntax-entity", |r| {
            r.color("var(--syntax-entity)")
        })
        .rule(".code-block .syntax-string", |r| {
            r.color("var(--syntax-string)")
        })
        .rule(".code-block .syntax-comment", |r| {
            r.color("var(--syntax-comment)")
                .custom("font-style", "italic")
        })
        .rule(".code-block .syntax-constant", |r| {
            r.color("var(--syntax-constant)")
        })
        .rule(".code-block .syntax-storage", |r| {
            r.color("var(--syntax-storage)")
        })
        .rule(".code-block .syntax-punctuation", |r| {
            r.color("var(--syntax-punctuation)")
        })
        .rule(".code-block .syntax-variable", |r| {
            r.color("var(--syntax-variable)")
        })
        .rule(".code-block .syntax-function", |r| {
            r.color("var(--syntax-function)")
        })
        .rule(".code-block .syntax-operator", |r| {
            r.color("var(--syntax-operator)")
        })
        .rule(".code-block .syntax-macro", |r| {
            r.color("var(--syntax-macro)")
        })
        .rule("nav", |r| {
            r.position("sticky")
                .top("0")
                .background("var(--nav-background)")
                .backdrop_filter("blur(8px)")
                .height("calc(var(--scale-3) + var(--scale-n2))")
                .padding("0 var(--scale-1)")
                .border_bottom("1px solid var(--border)")
                .display("flex")
                .align_items("center")
                .custom("z-index", "100")
                .transition("background-color 0.3s ease, border-color 0.3s ease")
        })
        .rule(".nav-logo", |r| {
            r.display("flex")
                .align_items("center")
                .custom("flex-shrink", "0")
        })
        .rule(".nav-logo img", |r| r.height("var(--scale-2)"))
        .rule(".nav-links", |r| {
            r.display("flex")
                .align_items("center")
                .gap("var(--scale-n1)")
                .custom("margin-left", "auto")
        })
        .rule(".nav-links > a, .nav-dropdown > a", |r| {
            r.color("var(--text-secondary)")
                .font_size("var(--scale-0)")
                .font_weight("500")
                .padding("var(--scale-n2) var(--scale-n1)")
                .white_space("nowrap")
                .custom("letter-spacing", "0.01em")
        })
        .rule(".nav-links > a:hover, .nav-dropdown > a:hover", |r| {
            r.color("var(--text)").text_decoration("none")
        })
        .rule(".nav-dropdown", |r| {
            r.position("relative").display("flex").align_items("center")
        })
        .rule(".nav-dropdown-menu", |r| {
            r.display("none")
                .position("absolute")
                .top("100%")
                .left("0")
                .background("var(--background)")
                .border("1px solid var(--border)")
                .border_radius("6px")
                .custom("box-shadow", "0 4px 12px #0000001a")
                .padding("var(--scale-n2) 0")
                .custom("min-width", "160px")
                .custom("z-index", "200")
        })
        .rule(
            ".nav-dropdown:hover .nav-dropdown-menu, .nav-dropdown:focus-within .nav-dropdown-menu",
            |r| r.display("block"),
        )
        .rule(".nav-dropdown-menu a", |r| {
            r.display("block")
                .padding("var(--scale-n2) var(--scale-0)")
                .color("var(--text-secondary)")
                .font_size("var(--scale-0)")
        })
        .rule(".nav-dropdown-menu a:hover", |r| {
            r.background("var(--code-background)")
                .color("var(--text)")
                .text_decoration("none")
        })
        .rule(".nav-dropdown-menu hr", |r| r.margin("var(--scale-n2) 0"))
        .rule(".layout", |r| {
            r.display("grid")
                .custom("grid-template-columns", &grid())
                .min_height("calc(100vh - calc(var(--scale-3) + var(--scale-n2)))")
        })
        .rule(".sidebar", |r| {
            r.position("sticky")
                .top("calc(var(--scale-3) + var(--scale-n2))")
                .height("calc(100vh - calc(var(--scale-3) + var(--scale-n2)))")
                .custom("overflow-y", "auto")
                .padding("var(--scale-1) var(--scale-0)")
                .custom("border-right", "1px solid var(--border)")
        })
        .rule(".sidebar a", |r| {
            r.display("block")
                .padding("var(--scale-n2) var(--scale-n1)")
                .color("var(--text-secondary)")
                .font_size("var(--scale-n0h)")
                .border_radius("4px")
        })
        .rule(".sidebar a:hover", |r| {
            r.color("var(--text)")
                .background("var(--code-background)")
                .text_decoration("none")
        })
        .rule(".sidebar a[aria-current=\"page\"]", |r| {
            r.color("var(--accent)")
                .background("var(--code-background)")
        })
        .rule(".outline", |r| {
            r.position("sticky")
                .top("calc(var(--scale-3) + var(--scale-n2))")
                .height("calc(100vh - calc(var(--scale-3) + var(--scale-n2)))")
                .custom("overflow-y", "auto")
                .padding("var(--scale-1) var(--scale-0)")
                .border_left("1px solid var(--border)")
        })
        .rule(".outline-label", |r| {
            r.font_size("var(--scale-n0h)")
                .font_weight("500")
                .custom("text-transform", "uppercase")
                .custom("letter-spacing", "0.08em")
                .color("var(--text-secondary)")
                .margin_bottom("var(--scale-n1)")
        })
        .rule(".outline a", |r| {
            r.display("block")
                .padding("var(--scale-n2) var(--scale-n1)")
                .color("var(--text-secondary)")
                .font_size("var(--scale-n0h)")
                .border_left("2px solid transparent")
                .transition("color 0.2s, border-left-color 0.2s")
        })
        .rule(".outline a:hover", |r| {
            r.color("var(--text)").text_decoration("none")
        })
        .rule(".outline a[data-depth=\"0\"]", |r| {
            r.font_weight("500")
                .custom("text-transform", "uppercase")
                .custom("letter-spacing", "0.08em")
        })
        .rule(".outline a:first-child", |r| r.custom("padding-top", "0"))
        .rule(".outline a[data-depth=\"1\"]", |r| {
            r.padding_left("var(--scale-0)")
        })
        .rule(".outline a[data-depth=\"2\"]", |r| {
            r.padding_left("var(--scale-1)")
        })
        .rule(".outline a[data-depth=\"3\"]", |r| {
            r.padding_left("var(--scale-2)")
        })
        .rule(".outline a[data-depth=\"4\"]", |r| {
            r.padding_left("var(--scale-3)")
        })
        .rule(".outline a.active", |r| {
            r.color("var(--accent)")
                .custom("border-left-color", "var(--accent)")
        })
        .rule(".sidebar-label", |r| {
            r.font_size("var(--scale-n0h)")
                .font_weight("500")
                .custom("text-transform", "uppercase")
                .custom("letter-spacing", "0.08em")
                .color("var(--text-secondary)")
                .margin_top("var(--scale-1)")
                .margin_bottom("var(--scale-n1)")
        })
        .rule(".sidebar-label:first-child", |r| r.margin_top("0"))
        .rule(".hamburger", |r| {
            r.display("none")
                .background("transparent")
                .border("none")
                .cursor("pointer")
                .padding("0")
                .align_items("center")
                .justify_content("center")
                .custom("align-self", "center")
                .height("var(--scale-2)")
                .width("var(--scale-2)")
                .custom("flex-direction", "column")
                .gap("3px")
        })
        .rule(".hamburger span", |r| {
            r.display("block")
                .width("16px")
                .height("2px")
                .background("var(--text-secondary)")
                .border_radius("1px")
                .transition("background-color 0.3s ease")
        })
        .rule(".hamburger:hover span", |r| r.background("var(--text)"))
        .rule("table", |r| {
            r.width("100%")
                .border_collapse("collapse")
                .margin_bottom("var(--scale-0)")
        })
        .rule("th", |r| {
            r.text_align("left")
                .padding("var(--scale-n1)")
                .border_bottom("2px solid var(--border)")
                .font_weight("600")
                .font_size("var(--scale-n1)")
                .custom("text-transform", "uppercase")
                .custom("letter-spacing", "0.02em")
                .color("var(--text-secondary)")
        })
        .rule("td", |r| {
            r.padding("var(--scale-n1)")
                .border_bottom("1px solid var(--border)")
        })
        .rule("tbody tr:nth-child(even)", |r| {
            r.background("var(--table-stripe)")
        })
        .rule("blockquote", |r| {
            r.border_left("3px solid var(--accent)")
                .padding_left("var(--scale-0)")
                .color("var(--text-secondary)")
                .margin_bottom("var(--scale-0)")
                .custom("font-style", "italic")
        })
        .rule("hr", |r| {
            r.border("none")
                .border_bottom("1px solid var(--border)")
                .margin("var(--scale-1) 0")
        })
        .rule("img", |r| {
            r.max_width("100%").height("auto").display("block")
        })
        .rule(".center", |r| r.text_align("center").margin("0 auto"))
        .rule(".subtitle", |r| {
            r.color("var(--text-secondary)")
                .font_size("var(--scale-1)")
                .font_weight("400")
                .custom("letter-spacing", "-0.01em")
                .margin_bottom("var(--scale-1)")
                .display("block")
        })
        .rule("a.subtitle:hover", |r| {
            r.color("var(--text)").text_decoration("none")
        })
        .rule("ul, ol", |r| {
            r.padding_left("var(--scale-1)")
                .margin_bottom("var(--scale-0)")
        })
        .rule("li", |r| r.margin_bottom("var(--scale-n2)"))
        .rule("dl", |r| r.margin_bottom("var(--scale-0)"))
        .rule("dt", |r| {
            r.font_weight("600")
                .margin_top("var(--scale-n1)")
                .custom("letter-spacing", "-0.01em")
        })
        .rule("dd", |r| {
            r.margin_bottom("var(--scale-n1)")
                .padding_left("var(--scale-0)")
        })
        .rule("footer", |r| {
            r.text_align("center")
                .padding("var(--scale-2) 0 var(--scale-1) 0")
                .margin_top("var(--scale-2)")
                .custom("border-top", "1px solid var(--border)")
                .color("var(--text-secondary)")
                .font_size("var(--scale-n1)")
                .custom("letter-spacing", "0.02em")
        })
        .rule(".footer-icon", |r| {
            r.display("inline-block")
                .color("var(--text-secondary)")
                .margin_top("var(--scale-n1)")
                .transition("color 0.2s")
        })
        .rule(".footer-icon:hover", |r| {
            r.color("var(--text)").text_decoration("none")
        })
        .rule(".footer-icon svg", |r| r.display("block"))
        .rule(".copy-button", |r| {
            r.position("absolute")
                .top("8px")
                .right("8px")
                .background("var(--nav-background)")
                .border("1px solid var(--border)")
                .border_radius("4px")
                .padding("2px 8px")
                .color("var(--text-secondary)")
                .cursor("pointer")
                .font_size("var(--scale-0)")
                .opacity("0")
                .transition("opacity 0.2s")
        })
        .rule(".code-block:hover .copy-button", |r| r.opacity("1"))
        .rule(".theme-toggle", |r| {
            r.background("transparent")
                .border("none")
                .cursor("pointer")
                .font_size("var(--scale-0)")
                .width("1.5rem")
                .height("1.5rem")
                .custom("flex-shrink", "0")
                .display("flex")
                .align_items("center")
                .justify_content("center")
                .overflow("hidden")
                .padding("0")
                .color("var(--text-secondary)")
                .transition("color 0.2s")
        })
        .rule(".theme-toggle svg", |r| {
            r.width("var(--scale-0)").height("var(--scale-0)")
        })
        .rule(".theme-toggle:hover", |r| r.color("var(--text)"))
        .rule(".enhanced [data-animate]", |r| {
            r.opacity("0")
                .transform("translateY(20px)")
                .transition("opacity 0.6s ease, transform 0.6s ease")
        })
        .rule(".enhanced [data-visible]", |r| {
            r.opacity("1").transform("translateY(0)")
        })
        .rule("html[data-theme=\"dark\"]", dark)
        .rule("html[data-theme=\"dark\"] .nav-dropdown-menu", |r| {
            r.custom("box-shadow", "0 4px 12px #00000066")
        })
        .rule("html[data-theme=\"light\"]", |r| {
            palette::PALETTE
                .iter()
                .chain(palette::SYNTAX)
                .fold(r, |p, token| {
                    p.custom(&format!("--{}", token.role), token.light)
                })
        })
        .media("prefers-color-scheme: dark", |m| {
            palette::PALETTE
                .iter()
                .chain(palette::SYNTAX)
                .fold(m, |s, token| {
                    s.variable(&format!("--{}", token.role), token.dark)
                })
                .rule(".nav-dropdown-menu", |r| {
                    r.custom("box-shadow", "0 4px 12px #00000066")
                })
        })
        .media("max-width: 1280px", |m| {
            m.rule(".outline", |r| r.display("none"))
                .rule(".layout", |r| {
                    r.custom(
                        "grid-template-columns",
                        &format!("{}fr 1fr", proportion::scale(-3)),
                    )
                })
        })
        .media("max-width: 1024px", |m| {
            m.rule(".sidebar", |r| {
                r.display("none")
                    .position("fixed")
                    .top("calc(var(--scale-3) + var(--scale-n2))")
                    .left("0")
                    .width("280px")
                    .height("calc(100vh - calc(var(--scale-3) + var(--scale-n2)))")
                    .background("var(--background)")
                    .custom("z-index", "150")
                    .custom("border-right", "1px solid var(--border)")
            })
            .rule(".sidebar.open", |r| r.display("block"))
            .rule(".layout", |r| r.custom("grid-template-columns", "1fr"))
            .rule(".hamburger", |r| r.display("flex"))
        })
        .media("max-width: 768px", |m| {
            m.rule("main", |r| r.padding("var(--scale-0) var(--scale-n1)"))
                .rule("h1", |r| {
                    r.font_size("var(--scale-2)")
                        .custom("letter-spacing", "-0.02em")
                })
                .rule("nav", |r| r.padding("0 var(--scale-n1)"))
        })
}

pub trait Composition {
    #[must_use]
    fn heading(self, level: u8, text: &str) -> Self;
    #[must_use]
    fn image(self, src: &str, alt: &str) -> Self;
    #[must_use]
    fn title(self, text: &str) -> Self;
    #[must_use]
    fn subtitle(self, text: &str) -> Self;
    #[must_use]
    fn rule(self) -> Self;
    #[must_use]
    fn paragraph(self, f: impl FnOnce(Span) -> Span) -> Self;
    #[must_use]
    fn section(self, title: &str, f: impl FnOnce(Body) -> Body) -> Self;
    #[must_use]
    fn subsection(self, title: &str, f: impl FnOnce(Body) -> Body) -> Self;
    #[must_use]
    fn term(self, word: &str, f: impl FnOnce(Span) -> Span) -> Self;
    #[must_use]
    fn aside(self, f: impl FnOnce(Span) -> Span) -> Self;
    #[must_use]
    fn navigation(self, f: impl FnOnce(Body) -> Body) -> Self;
    #[must_use]
    fn literal(self, source: &str, language: Language) -> Self;
    #[must_use]
    fn highlight(self, html: &str, language: Language) -> Self;
    #[must_use]
    fn shell(self, command: &str) -> Self;
    #[must_use]
    fn bold(self, text: &str) -> Self;
    #[must_use]
    fn link(self, href: &str, text: &str) -> Self;
    #[must_use]
    fn code(self, name: &str, language: Language) -> Self;
}

impl Composition for Body {
    #[trace(channels = [document])]
    fn heading(self, level: u8, text: &str) -> Self {
        self.element(&format!("h{}", level.clamp(1, 6)), |e| e.text(text))
    }

    #[trace(channels = [document])]
    fn image(self, src: &str, alt: &str) -> Self {
        self.element("div", |d| {
            d.class("center")
                .element("img", |e| e.attribute("src", src).attribute("alt", alt))
        })
    }

    #[trace(channels = [document])]
    fn title(self, text: &str) -> Self {
        self.heading(1, text)
    }

    #[trace(channels = [document])]
    fn subtitle(self, text: &str) -> Self {
        self.element("p", |p| p.class("subtitle").text(text))
    }

    #[trace(channels = [document])]
    fn rule(self) -> Self {
        self.element("hr", |e| e)
    }

    #[trace(channels = [document])]
    fn paragraph(self, f: impl FnOnce(Span) -> Span) -> Self {
        self.element("p", |p| p.span(f))
    }

    #[trace(channels = [document])]
    fn section(self, title: &str, f: impl FnOnce(Body) -> Body) -> Self {
        self.element("section", |s| f(s.heading(2, title)))
    }

    #[trace(channels = [document])]
    fn subsection(self, title: &str, f: impl FnOnce(Body) -> Body) -> Self {
        self.element("section", |s| f(s.heading(3, title)))
    }

    #[trace(channels = [document])]
    fn term(self, word: &str, f: impl FnOnce(Span) -> Span) -> Self {
        self.element("dl", |dl| {
            dl.element("dt", |dt| dt.text(word))
                .element("dd", |dd| dd.span(f))
        })
    }

    #[trace(channels = [document])]
    fn aside(self, f: impl FnOnce(Span) -> Span) -> Self {
        self.element("blockquote", |bq| bq.element("p", |p| p.span(f)))
    }

    #[trace(channels = [document])]
    fn navigation(self, f: impl FnOnce(Body) -> Body) -> Self {
        self.element("nav", f)
    }

    #[trace(channels = [document])]
    fn literal(self, source: &str, language: Language) -> Self {
        body::Body::literal(self, source, language)
    }

    #[trace(channels = [document])]
    fn highlight(self, html: &str, language: Language) -> Self {
        body::Body::highlight(self, html, language)
    }

    #[trace(channels = [document])]
    fn shell(self, command: &str) -> Self {
        self.literal(command, Language::Bash)
    }

    #[trace(channels = [document])]
    fn bold(self, text: &str) -> Self {
        self.element("strong", |e| e.text(text))
    }

    #[trace(channels = [document])]
    fn link(self, href: &str, text: &str) -> Self {
        self.element("a", |e| e.attribute("href", href).text(text))
    }

    #[trace(channels = [document])]
    fn code(self, name: &str, language: Language) -> Self {
        body::Body::code(self, name, language)
    }
}
