use body::Body;
use element::Language;
use observe::trace;
use page::Page;
use span::Span;
use style::Style;

const PHI: f64 = 1.618_033_988_749_895;

#[trace(channels = [document])]
fn scale(k: i32) -> String {
    format!("{}rem", PHI.powi(k))
}

fn grid() -> String {
    let left = PHI.powi(-3);
    let right = PHI.powi(-2);
    format!("{left}fr 1fr {right}fr")
}

const LIGHT: &[(&str, &str)] = &[
    ("--background", "#fafaf9"),
    ("--text", "#1a1a1a"),
    ("--text-secondary", "#6b7280"),
    ("--accent", "#d45d00"),
    ("--accent-hover", "#b84e00"),
    ("--border", "#e5e5e5"),
    ("--code-background", "#f5f5f4"),
    ("--code-text", "#1a1a1a"),
    ("--nav-background", "rgba(250, 250, 249, 0.8)"),
    ("--table-stripe", "#f9fafb"),
];

const DARK: &[(&str, &str)] = &[
    ("--background", "#0f0f0f"),
    ("--text", "#e5e5e5"),
    ("--text-secondary", "#9ca3af"),
    ("--accent", "#ff8c42"),
    ("--accent-hover", "#ffa366"),
    ("--border", "#2e2e2e"),
    ("--code-background", "#1e1e1e"),
    ("--code-text", "#e5e5e5"),
    ("--nav-background", "rgba(15, 15, 15, 0.8)"),
    ("--table-stripe", "#1a1a1a"),
];

#[trace(channels = [document])]
fn palette(style: Style, values: &[(&str, &str)]) -> Style {
    values
        .iter()
        .fold(style, |s, &(name, value)| s.variable(name, value))
}

#[trace(channels = [document])]
fn overrides(properties: style::Properties, values: &[(&str, &str)]) -> style::Properties {
    values
        .iter()
        .fold(properties, |p, &(name, value)| p.custom(name, value))
}

#[trace(channels = [document])]
pub fn page(
    arguments: &render::Arguments,
    title: &str,
    context: &str,
    identifier: &str,
    f: impl FnOnce(Body) -> Body,
) -> miette::Result<()> {
    let root = arguments.root();
    render::generate(
        arguments,
        Page::new()
            .title(&format!("{title} \u{2014} Vantle"))
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
                            .element("a", |a| {
                                a.attribute("href", &format!("{root}system/generation/"))
                                    .text("Generation")
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
                        .element("aside", |a| {
                            a.class("sidebar")
                                .attribute("aria-label", "Page navigation")
                        })
                        .element("main", |m| {
                            f(m).element("footer", |footer| {
                                footer.element("p", |p| {
                                    p.span(|s| {
                                        s.text("\u{00a9} 2025 Vantle \u{00b7} ")
                                            .link("https://vantle.org", "@robert.vanderzee")
                                    })
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

#[trace(channels = [document])]
#[must_use]
pub fn theme() -> Style {
    palette(Style::new(), LIGHT)
        .variable("--phi", &PHI.to_string())
        .variable("--scale-n2", &scale(-2))
        .variable("--scale-n1", &scale(-1))
        .variable("--scale-0", &scale(0))
        .variable("--scale-1", &scale(1))
        .variable("--scale-2", &scale(2))
        .variable("--scale-3", &scale(3))
        .rule("*", |r| r.margin("0").padding("0").box_sizing("border-box"))
        .rule("body", |r| {
            r.font_family("-apple-system, BlinkMacSystemFont, 'Segoe UI', system-ui, sans-serif")
                .background("var(--background)")
                .color("var(--text)")
                .line_height(&PHI.to_string())
                .font_size("var(--scale-0)")
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
        })
        .rule("pre", |r| {
            r.background("var(--code-background)")
                .border_radius("6px")
                .padding("var(--scale-0)")
                .overflow("auto")
                .margin_bottom("var(--scale-0)")
                .line_height("1.5")
                .position("relative")
        })
        .rule("pre code", |r| {
            r.background("transparent")
                .padding("0")
                .font_size("var(--scale-0)")
        })
        .rule(".code-block", |r| {
            r.position("relative").margin_bottom("var(--scale-0)")
        })
        .rule(".code-block pre", |r| r.margin("0"))
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
                .font_size("var(--scale-n1)")
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
                .custom("box-shadow", "0 4px 12px rgba(0, 0, 0, 0.1)")
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
                .font_size("var(--scale-n1)")
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
                .font_size("var(--scale-n1)")
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
            r.font_size("var(--scale-n1)")
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
                .font_size("var(--scale-n1)")
                .border_left("2px solid transparent")
        })
        .rule(".outline a:hover", |r| {
            r.color("var(--text)").text_decoration("none")
        })
        .rule(".outline a[data-depth=\"3\"]", |r| {
            r.padding_left("var(--scale-0)")
        })
        .rule(".outline a[data-depth=\"4\"]", |r| {
            r.padding_left("var(--scale-1)")
        })
        .rule(".outline a.active", |r| {
            r.color("var(--accent)")
                .custom("border-left-color", "var(--accent)")
        })
        .rule(".sidebar-label", |r| {
            r.font_size("var(--scale-n1)")
                .font_weight("500")
                .custom("text-transform", "uppercase")
                .custom("letter-spacing", "0.08em")
                .color("var(--text-secondary)")
                .margin_top("var(--scale-1)")
                .margin_bottom("var(--scale-n1)")
        })
        .rule(".hamburger", |r| {
            r.display("none")
                .background("transparent")
                .border("none")
                .cursor("pointer")
                .font_size("var(--scale-1)")
                .color("var(--text-secondary)")
                .padding("var(--scale-n2)")
        })
        .rule(".hamburger:hover", |r| r.color("var(--text)"))
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
                .font_size("var(--scale-n2)")
                .opacity("0")
                .transition("opacity 0.2s")
        })
        .rule(".code-block:hover .copy-button", |r| r.opacity("1"))
        .rule(".theme-toggle", |r| {
            r.background("transparent")
                .border("none")
                .cursor("pointer")
                .font_size("var(--scale-0)")
                .padding("0")
                .color("var(--text-secondary)")
                .transition("color 0.2s")
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
        .rule("html[data-theme=\"dark\"]", |r| overrides(r, DARK))
        .rule("html[data-theme=\"dark\"] .nav-dropdown-menu", |r| {
            r.custom("box-shadow", "0 4px 12px rgba(0, 0, 0, 0.4)")
        })
        .rule("html[data-theme=\"light\"]", |r| overrides(r, LIGHT))
        .media("prefers-color-scheme: dark", |m| {
            palette(m, DARK).rule(".nav-dropdown-menu", |r| {
                r.custom("box-shadow", "0 4px 12px rgba(0, 0, 0, 0.4)")
            })
        })
        .media("max-width: 1280px", |m| {
            m.rule(".outline", |r| r.display("none"))
                .rule(".layout", |r| {
                    r.custom("grid-template-columns", &format!("{}fr 1fr", PHI.powi(-3)))
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
            .rule(".hamburger", |r| r.display("block"))
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

#[trace(channels = [document])]
#[must_use]
pub fn license(body: Body) -> Body {
    body.heading(2, "Apache License")
        .paragraph(|p| p.text("Version 2.0, January 2004"))
        .paragraph(|p| {
            p.link(
                "https://www.apache.org/licenses/",
                "https://www.apache.org/licenses/",
            )
        })
        .paragraph(|p| {
            p.text("Vantle Research | Robert Scott VanDerzee | @robbie-vanderzee | people.vantle.org/robbie-vanderzee")
        })
        .rule()
        .section("Terms and Conditions", |s| {
            s.subsection("1. Definitions", |ss| {
                ss.term("License", |t| {
                    t.text("shall mean the terms and conditions for use, reproduction, and distribution as defined by Sections 1 through 9 of this document.")
                })
                .term("Licensor", |t| {
                    t.text("shall mean the copyright owner or entity authorized by the copyright owner that is granting the License.")
                })
                .term("Legal Entity", |t| {
                    t.text("shall mean the union of the acting entity and all other entities that control, are controlled by, or are under common control with that entity. For the purposes of this definition, \u{201c}control\u{201d} means (i) the power, direct or indirect, to cause the direction or management of such entity, whether by contract or otherwise, or (ii) ownership of fifty percent (50%) or more of the outstanding shares, or (iii) beneficial ownership of such entity.")
                })
                .term("You", |t| {
                    t.text("(or \u{201c}Your\u{201d}) shall mean an individual or Legal Entity exercising permissions granted by this License.")
                })
                .term("Source", |t| {
                    t.text("form shall mean the preferred form for making modifications, including but not limited to software source code, documentation source, and configuration files.")
                })
                .term("Object", |t| {
                    t.text("form shall mean any form resulting from mechanical transformation or translation of a Source form, including but not limited to compiled object code, generated documentation, and conversions to other media types.")
                })
                .term("Work", |t| {
                    t.text("shall mean the work of authorship, whether in Source or Object form, made available under the License, as indicated by a copyright notice that is included in or attached to the work (an example is provided in the Appendix below).")
                })
                .term("Derivative Works", |t| {
                    t.text("shall mean any work, whether in Source or Object form, that is based on (or derived from) the Work and for which the editorial revisions, annotations, elaborations, or other modifications represent, as a whole, an original work of authorship. For the purposes of this License, Derivative Works shall not include works that remain separable from, or merely link (or bind by name) to the interfaces of, the Work and Derivative Works thereof.")
                })
                .term("Contribution", |t| {
                    t.text("shall mean any work of authorship, including the original version of the Work and any modifications or additions to that Work or Derivative Works thereof, that is intentionally submitted to Licensor for inclusion in the Work by the copyright owner or by an individual or Legal Entity authorized to submit on behalf of the copyright owner. For the purposes of this definition, \u{201c}submitted\u{201d} means any form of electronic, verbal, or written communication sent to the Licensor or its representatives, including but not limited to communication on electronic mailing lists, source code control systems, and issue tracking systems that are managed by, or on behalf of, the Licensor for the purpose of discussing and improving the Work, but excluding communication that is conspicuously marked or otherwise designated in writing by the copyright owner as \u{201c}Not a Contribution.\u{201d}")
                })
                .term("Contributor", |t| {
                    t.text("shall mean Licensor and any individual or Legal Entity on behalf of whom a Contribution has been received by Licensor and subsequently incorporated within the Work.")
                })
            })
            .subsection("2. Grant of Copyright License", |ss| {
                ss.paragraph(|p| {
                    p.text("Subject to the terms and conditions of this License, each Contributor hereby grants to You a perpetual, worldwide, non-exclusive, no-charge, royalty-free, irrevocable copyright license to reproduce, prepare Derivative Works of, publicly display, publicly perform, sublicense, and distribute the Work and such Derivative Works in Source or Object form.")
                })
            })
            .subsection("3. Grant of Patent License", |ss| {
                ss.paragraph(|p| {
                    p.text("Subject to the terms and conditions of this License, each Contributor hereby grants to You a perpetual, worldwide, non-exclusive, no-charge, royalty-free, irrevocable (except as stated in this section) patent license to make, have made, use, offer to sell, sell, import, and otherwise transfer the Work, where such license applies only to those patent claims licensable by such Contributor that are necessarily infringed by their Contribution(s) alone or by combination of their Contribution(s) with the Work to which such Contribution(s) was submitted. If You institute patent litigation against any entity (including a cross-claim or counterclaim in a lawsuit) alleging that the Work or a Contribution incorporated within the Work constitutes direct or contributory patent infringement, then any patent licenses granted to You under this License for that Work shall terminate as of the date such litigation is filed.")
                })
            })
            .subsection("4. Redistribution", |ss| {
                ss.paragraph(|p| {
                    p.text("You may reproduce and distribute copies of the Work or Derivative Works thereof in any medium, with or without modifications, and in Source or Object form, provided that You meet the following conditions:")
                })
                .element("ul", |ul| {
                    ul.element("li", |li| {
                        li.span(|s| s.text("You must give any other recipients of the Work or Derivative Works a copy of this License; and"))
                    })
                    .element("li", |li| {
                        li.span(|s| s.text("You must cause any modified files to carry prominent notices stating that You changed the files; and"))
                    })
                    .element("li", |li| {
                        li.span(|s| s.text("You must retain, in the Source form of any Derivative Works that You distribute, all copyright, patent, trademark, and attribution notices from the Source form of the Work, excluding those notices that do not pertain to any part of the Derivative Works; and"))
                    })
                    .element("li", |li| {
                        li.span(|s| s.text("If the Work includes a \u{201c}NOTICE\u{201d} text file as part of its distribution, then any Derivative Works that You distribute must include a readable copy of the attribution notices contained within such NOTICE file, excluding those notices that do not pertain to any part of the Derivative Works, in at least one of the following places: within a NOTICE text file distributed as part of the Derivative Works; within the Source form or documentation, if provided along with the Derivative Works; or, within a display generated by the Derivative Works, if and wherever such third-party notices normally appear. The contents of the NOTICE file are for informational purposes only and do not modify the License. You may add Your own attribution notices within Derivative Works that You distribute, alongside or as an addendum to the NOTICE text from the Work, provided that such additional attribution notices cannot be construed as modifying the License."))
                    })
                })
                .paragraph(|p| {
                    p.text("You may add Your own copyright statement to Your modifications and may provide additional or different license terms and conditions for use, reproduction, or distribution of Your modifications, or for any such Derivative Works as a whole, provided Your use, reproduction, and distribution of the Work otherwise complies with the conditions stated in this License.")
                })
            })
            .subsection("5. Submission of Contributions", |ss| {
                ss.paragraph(|p| {
                    p.text("Unless You explicitly state otherwise, any Contribution intentionally submitted for inclusion in the Work by You to the Licensor shall be under the terms and conditions of this License, without any additional terms or conditions. Notwithstanding the above, nothing herein shall supersede or modify the terms of any separate license agreement you may have executed with Licensor regarding such Contributions.")
                })
            })
            .subsection("6. Trademarks", |ss| {
                ss.paragraph(|p| {
                    p.text("This License does not grant permission to use the trade names, trademarks, service marks, or product names of the Licensor, except as required for reasonable and customary use in describing the origin of the Work and reproducing the content of the NOTICE file.")
                })
            })
            .subsection("7. Disclaimer of Warranty", |ss| {
                ss.paragraph(|p| {
                    p.text("Unless required by applicable law or agreed to in writing, Licensor provides the Work (and each Contributor provides its Contributions) on an \u{201c}AS IS\u{201d} BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied, including, without limitation, any warranties or conditions of TITLE, NON-INFRINGEMENT, MERCHANTABILITY, or FITNESS FOR A PARTICULAR PURPOSE. You are solely responsible for determining the appropriateness of using or redistributing the Work and assume any risks associated with Your exercise of permissions under this License.")
                })
            })
            .subsection("8. Limitation of Liability", |ss| {
                ss.paragraph(|p| {
                    p.text("In no event and under no legal theory, whether in tort (including negligence), contract, or otherwise, unless required by applicable law (such as deliberate and grossly negligent acts) or agreed to in writing, shall any Contributor be liable to You for damages, including any direct, indirect, special, incidental, or consequential damages of any character arising as a result of this License or out of the use or inability to use the Work (including but not limited to damages for loss of goodwill, work stoppage, computer failure or malfunction, or any and all other commercial damages or losses), even if such Contributor has been advised of the possibility of such damages.")
                })
            })
            .subsection("9. Accepting Warranty or Additional Liability", |ss| {
                ss.paragraph(|p| {
                    p.text("While redistributing the Work or Derivative Works thereof, You may choose to offer, and charge a fee for, acceptance of support, warranty, indemnity, or other liability obligations and/or rights consistent with this License. However, in accepting such obligations, You may act only on Your own behalf and on Your sole responsibility, not on behalf of any other Contributor, and only if You agree to indemnify, defend, and hold each Contributor harmless for any liability incurred by, or claims asserted against, such Contributor by reason of your accepting any such warranty or additional liability.")
                })
            })
        })
        .rule()
        .element("p", |p| {
            p.class("center")
                .text("END OF TERMS AND CONDITIONS")
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
    fn shell(self, command: &str) -> Self {
        self.element("pre", |p| {
            p.element("code", |c| c.class("language-bash").text(command))
        })
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
