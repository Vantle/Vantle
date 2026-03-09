use element::Element;
use extraction::Query;

#[must_use]
pub fn page(root: &str) -> page::Page {
    navigation::layout("Web", &index::generation::web::web(root), root, |c| {
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
                        .code(".page.rs")
                        .text(" content libraries using the ")
                        .code("Composition")
                        .text(" trait, which provides semantic methods for document structure:")
                })
                .extract(function_document_extract::EXTRACTIONS.one())
                .section("Composition", |ss| {
                    ss.paragraph(|p| {
                        p.text("The ")
                            .code("Composition")
                            .text(" trait adds high-level semantic methods to the body builder:")
                    })
                    .list(|ul| {
                        ul.item(|s| {
                            s.code("heading")
                                .text(", ")
                                .code("title")
                                .text(", ")
                                .code("subtitle")
                                .text(", ")
                                .code("section")
                                .text(" \u{2014} document structure")
                        })
                        .item(|s| {
                            s.code("paragraph")
                                .text(", ")
                                .code("aside")
                                .text(", ")
                                .code("rule")
                                .text(", ")
                                .code("term")
                                .text(", ")
                                .code("definition")
                                .text(" \u{2014} content blocks")
                        })
                        .item(|s| {
                            s.code("navigation")
                                .text(", ")
                                .code("figure")
                                .text(" \u{2014} navigation")
                        })
                        .item(|s| {
                            s.code("bold")
                                .text(", ")
                                .code("link")
                                .text(" \u{2014} inline formatting")
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
                .list(|ul| {
                    ul.item(|s| {
                        s.bold("Proportions")
                            .text(": phi-scaled spacing variables from ")
                            .code("--scale-n2")
                            .text(" to ")
                            .code("--scale-3")
                    })
                    .feature(
                        "Typography",
                        ": heading sizes follow the golden ratio scale",
                    )
                    .feature(
                        "Layout",
                        ": three-column grid (sidebar, main, outline) with responsive collapse",
                    )
                    .feature(
                        "Themes",
                        ": light and dark via CSS custom properties, persisted in localStorage",
                    )
                })
            })
            .rule()
            .section("Highlight", |s| {
                s.paragraph(|p| {
                    p.text("Syntax highlighting dispatches to language-specific highlighters. Each produces HTML spans with semantic CSS classes for theme-aware coloring.")
                })
                .table(|t| {
                    t.header(["Language", "Strategy"])
                        .row(["Rust", "AST via syn, with snippet fallback"])
                        .row(["Molten", "Custom AST traversal"])
                        .row(["JSON", "Structural value coloring"])
                        .row(["Starlark", "Tree-sitter grammar"])
                        .row(["Bash", "Tree-sitter grammar"])
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
                .list(|ul| {
                    ul.feature("Navigate", ": SPA navigation without page reloads")
                        .feature("Theme", ": toggle button with localStorage persistence")
                        .feature("Clipboard", ": copy-to-clipboard on code blocks")
                        .feature("Outline", ": scroll-tracking table of contents")
                        .feature("Scroll", ": smooth scroll behavior")
                        .feature("Hamburger", ": mobile sidebar toggle")
                })
            })
            .rule()
            .section("Starlark", |s| {
                s.paragraph(|p| {
                    p.text("Bazel rules for authoring, publishing, verifying, and serving documentation. All rules are loaded from ")
                        .code("//component/web/starlark:defs.bzl")
                        .text(".")
                })
                .section("document", |ss| {
                    ss.paragraph(|p| {
                        p.text("Compiles a Rust DSL source into a binary, executes it, and collects the generated output. Standard dependencies (")
                            .code("html")
                            .text(", ")
                            .code("miette")
                            .text(") are injected automatically.")
                    })
                    .extract(info_document_rule::EXTRACTIONS.one())
                    .table(|t| {
                        t.header(["Parameter", "Description"])
                            .describe("src", "Rust binary source file (.document.rs)")
                            .describe("destination", "Workspace-relative output path")
                            .describe("deps", "Additional compile dependencies")
                            .describe("data", "Runtime data files for WASM or code injection")
                            .markup([
                                Element::Tag {
                                    name: "code".into(),
                                    attributes: Vec::new(),
                                    children: vec![Element::Text("compile_data".into())],
                                },
                                Element::Tag {
                                    name: "span".into(),
                                    attributes: Vec::new(),
                                    children: vec![
                                        Element::Text("Compile-time data files for ".into()),
                                        Element::Tag {
                                            name: "code".into(),
                                            attributes: Vec::new(),
                                            children: vec![Element::Text("include_str!".into())],
                                        },
                                        Element::Text(" sources".into()),
                                    ],
                                },
                            ])
                    })
                })
                .section("copy", |ss| {
                    ss.paragraph(|p| {
                        p.text("Wraps an existing file with a destination path for inclusion in folder manifests. Uses a zero-cost symlink action.")
                    })
                    .extract(copy_rule::EXTRACTIONS.one())
                })
                .section("folder", |ss| {
                    ss.paragraph(|p| {
                        p.text("Aggregates document and copy targets into a manifest, then copies all generated files into the workspace.")
                    })
                    .extract(folder_rule::EXTRACTIONS.one())
                })
                .section("distribute", |ss| {
                    ss.paragraph(|p| {
                        p.text("Serves the workspace directory over HTTP for local preview.")
                    })
                    .extract(distribute_rule::EXTRACTIONS.one())
                })
            })
    })
}
