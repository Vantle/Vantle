use style::Composition;
use web::element::Language;

fn main() -> miette::Result<()> {
    let arguments = html::Arguments::parse();
    style::page(&arguments, "Web", "generation", "web", |c| {
        c.title("Web")
            .subtitle("Web file generation by output format")
            .rule()
            .paragraph(|p| {
                p.text("Generates web documentation from Rust DSL sources. Each generator targets a specific output format and integrates with Bazel through Starlark rules for authoring, publishing, and serving.")
            })
            .rule()
            .section("HTML", |s| {
                s.paragraph(|p| {
                    p.text("Renders a ")
                        .code("Page")
                        .text(" element tree into complete HTML documents. Handles heading extraction for automatic table-of-contents generation, syntax-highlighted code blocks, and data injection for runtime assets.")
                })
                .paragraph(|p| {
                    p.text("Pages are authored as ")
                        .code(".document.rs")
                        .text(" files using the ")
                        .code("Composition")
                        .text(" trait, which provides semantic methods for document structure:")
                })
                .literal("use style::Composition;\n\nfn main() -> miette::Result<()> {\n    let arguments = html::Arguments::parse();\n    style::page(&arguments, \"Title\", \"context\", \"page\", |c| {\n        c.title(\"Title\")\n            .section(\"Overview\", |s| {\n                s.paragraph(|p| p.text(\"Content here.\"))\n            })\n    })\n}", Language::Rust)
                .subsection("Composition", |ss| {
                    ss.paragraph(|p| {
                        p.text("The ")
                            .code("Composition")
                            .text(" trait adds high-level semantic methods to the body builder:")
                    })
                    .element("ul", |ul| {
                        ul.element("li", |li| {
                            li.span(|s| {
                                s.code("title")
                                    .text(", ")
                                    .code("subtitle")
                                    .text(", ")
                                    .code("section")
                                    .text(", ")
                                    .code("subsection")
                                    .text(" \u{2014} document structure")
                            })
                        })
                        .element("li", |li| {
                            li.span(|s| {
                                s.code("paragraph")
                                    .text(", ")
                                    .code("aside")
                                    .text(", ")
                                    .code("rule")
                                    .text(", ")
                                    .code("term")
                                    .text(" \u{2014} content blocks")
                            })
                        })
                        .element("li", |li| {
                            li.span(|s| {
                                s.code("literal")
                                    .text(", ")
                                    .code("highlight")
                                    .text(", ")
                                    .code("shell")
                                    .text(" \u{2014} code presentation")
                            })
                        })
                        .element("li", |li| {
                            li.span(|s| {
                                s.code("link")
                                    .text(", ")
                                    .code("bold")
                                    .text(", ")
                                    .code("italic")
                                    .text(" \u{2014} inline formatting")
                            })
                        })
                    })
                })
            })
            .rule()
            .section("Style", |s| {
                s.paragraph(|p| {
                    p.text("Generates CSS from a golden-ratio design system. All spacing, typography, and responsive breakpoints scale from ")
                        .bold("PHI")
                        .text(" (1.618). Colors are defined as paired light/dark values, switchable via ")
                        .code("data-theme")
                        .text(" attribute.")
                })
                .element("ul", |ul| {
                    ul.element("li", |li| {
                        li.span(|s| {
                            s.bold("Proportions")
                                .text(": phi-scaled spacing variables from ")
                                .code("--scale-n2")
                                .text(" to ")
                                .code("--scale-3")
                        })
                    })
                    .element("li", |li| {
                        li.span(|s| {
                            s.bold("Typography")
                                .text(": heading sizes follow the golden ratio scale")
                        })
                    })
                    .element("li", |li| {
                        li.span(|s| {
                            s.bold("Layout")
                                .text(": three-column grid (sidebar, main, outline) with responsive collapse")
                        })
                    })
                    .element("li", |li| {
                        li.span(|s| {
                            s.bold("Themes")
                                .text(": light and dark via CSS custom properties, persisted in localStorage")
                        })
                    })
                })
            })
            .rule()
            .section("Highlight", |s| {
                s.paragraph(|p| {
                    p.text("Syntax highlighting dispatches to language-specific highlighters. Each produces HTML spans with semantic CSS classes for theme-aware coloring.")
                })
                .element("table", |t| {
                    t.element("thead", |h| {
                        h.element("tr", |r| {
                            r.element("th", |c| c.text("Language"))
                                .element("th", |c| c.text("Strategy"))
                        })
                    })
                    .element("tbody", |b| {
                        b.element("tr", |r| {
                            r.element("td", |c| c.text("Rust"))
                                .element("td", |c| c.text("AST via syn, with snippet fallback"))
                        })
                        .element("tr", |r| {
                            r.element("td", |c| c.text("Molten"))
                                .element("td", |c| c.text("Custom AST traversal"))
                        })
                        .element("tr", |r| {
                            r.element("td", |c| c.text("JSON"))
                                .element("td", |c| c.text("Structural value coloring"))
                        })
                        .element("tr", |r| {
                            r.element("td", |c| c.text("Starlark"))
                                .element("td", |c| c.text("Tree-sitter grammar"))
                        })
                        .element("tr", |r| {
                            r.element("td", |c| c.text("Bash"))
                                .element("td", |c| c.text("Tree-sitter grammar"))
                        })
                    })
                })
            })
            .rule()
            .section("Assembly", |s| {
                s.paragraph(|p| {
                    p.text("Compiles Rust to WebAssembly for client-side interactivity. The assembly chain produces ")
                        .code("compute.js")
                        .text(" and ")
                        .code("compute_bg.wasm")
                        .text(" via ")
                        .code("rust_wasm_bindgen")
                        .text(".")
                })
                .element("ul", |ul| {
                    ul.element("li", |li| {
                        li.span(|s| {
                            s.bold("Navigate")
                                .text(": SPA navigation without page reloads")
                        })
                    })
                    .element("li", |li| {
                        li.span(|s| {
                            s.bold("Theme")
                                .text(": toggle button with localStorage persistence")
                        })
                    })
                    .element("li", |li| {
                        li.span(|s| {
                            s.bold("Clipboard")
                                .text(": copy-to-clipboard on code blocks")
                        })
                    })
                    .element("li", |li| {
                        li.span(|s| {
                            s.bold("Outline")
                                .text(": scroll-tracking table of contents")
                        })
                    })
                    .element("li", |li| {
                        li.span(|s| {
                            s.bold("Scroll")
                                .text(": smooth scroll behavior")
                        })
                    })
                    .element("li", |li| {
                        li.span(|s| {
                            s.bold("Hamburger")
                                .text(": mobile sidebar toggle")
                        })
                    })
                })
            })
            .rule()
            .section("Starlark", |s| {
                s.paragraph(|p| {
                    p.text("Bazel rules for authoring, publishing, verifying, and serving documentation. All rules are loaded from ")
                        .code("//component/web/starlark:defs.bzl")
                        .text(".")
                })
                .subsection("document", |ss| {
                    ss.paragraph(|p| {
                        p.text("Compiles a Rust DSL source into a binary, executes it, and collects the generated output. Standard dependencies (")
                            .code("component:web")
                            .text(", ")
                            .code("html")
                            .text(", ")
                            .code("miette")
                            .text(") are injected automatically.")
                    })
                    .literal("document(\n    srcs = [\"page.document.rs\"],\n    destination = \"path/index.html\",\n    deps = [\"//system/generation/web:style\"],\n)", Language::Starlark)
                    .element("table", |t| {
                        t.element("thead", |h| {
                            h.element("tr", |r| {
                                r.element("th", |c| c.text("Parameter"))
                                    .element("th", |c| c.text("Description"))
                            })
                        })
                        .element("tbody", |b| {
                            b.element("tr", |r| {
                                r.element("td", |c| c.element("code", |c| c.text("srcs")))
                                    .element("td", |c| {
                                        c.text("Rust source files defining the page")
                                    })
                            })
                            .element("tr", |r| {
                                r.element("td", |c| c.element("code", |c| c.text("destination")))
                                    .element("td", |c| {
                                        c.text("Workspace-relative output path")
                                    })
                            })
                            .element("tr", |r| {
                                r.element("td", |c| c.element("code", |c| c.text("deps")))
                                    .element("td", |c| {
                                        c.text("Additional compile dependencies")
                                    })
                            })
                            .element("tr", |r| {
                                r.element("td", |c| c.element("code", |c| c.text("data")))
                                    .element("td", |c| {
                                        c.text("Runtime data files for WASM or code injection")
                                    })
                            })
                            .element("tr", |r| {
                                r.element("td", |c| {
                                    c.element("code", |c| c.text("compile_data"))
                                })
                                .element("td", |c| {
                                    c.text("Compile-time data files for ")
                                        .element("code", |c| c.text("include_str!"))
                                        .text(" sources")
                                })
                            })
                        })
                    })
                })
                .subsection("asset", |ss| {
                    ss.paragraph(|p| {
                        p.text("Wraps an existing file with a destination path for inclusion in publish manifests. Uses a zero-cost symlink action.")
                    })
                    .literal("asset(\n    name = \"document.compute.js\",\n    src = \":compute.js\",\n    destination = \"resource/system/document/compute.js\",\n)", Language::Starlark)
                })
                .subsection("publish", |ss| {
                    ss.paragraph(|p| {
                        p.text("Aggregates document and asset targets into a manifest, then copies all generated files into the workspace.")
                    })
                    .literal("publish(\n    name = \"publish.documentation\",\n    srcs = [\n        \":document.vantle\",\n        \"//system/documentation:document.stylesheet\",\n        \"//system/generation/web/assembly:document.compute.js\",\n    ],\n)", Language::Starlark)
                })
                .subsection("distribute", |ss| {
                    ss.paragraph(|p| {
                        p.text("Serves the workspace directory over HTTP for local preview.")
                    })
                    .literal("distribute(\n    name = \"distribute.documentation\",\n)", Language::Starlark)
                })
            })
    })
}
