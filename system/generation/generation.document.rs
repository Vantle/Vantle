use style::Composition;

fn main() -> miette::Result<()> {
    let arguments = html::Arguments::parse();
    let root = arguments.root();
    style::page(&arguments, "Generation", "generation", "generation", |c| {
        c.title("Generation")
            .subtitle("Code generation framework")
            .rule()
            .paragraph(|p| {
                p.text("The ")
                    .bold("Generation")
                    .text(" framework produces files from structured inputs. Each generator targets a specific output format and integrates with Bazel for incremental, cacheable builds.")
            })
            .rule()
            .section("Generators", |s| {
                s.subsection("Web", |ss| {
                    ss.aside(|a| {
                        a.italic("HTML, CSS, syntax highlighting, and WebAssembly from Rust DSL")
                    })
                    .paragraph(|p| {
                        p.text("Author documentation pages as ")
                            .code(".document.rs")
                            .text(" files using the ")
                            .code("Composition")
                            .text(" trait. The web generator renders complete HTML with automatic table-of-contents, syntax-highlighted code blocks, golden-ratio styling, and optional WASM interactivity.")
                    })
                    .paragraph(|p| {
                        p.link(&format!("{root}system/generation/web/"), "more \u{2192}")
                    })
                })
                .subsection("Autotest", |ss| {
                    ss.aside(|a| {
                        a.italic("JSON-driven test generation for Rust")
                    })
                    .paragraph(|p| {
                        p.text("Eliminate test boilerplate by generating Rust test functions from templates and JSON case definitions. Write the logic once, define test data declaratively, and produce exhaustive test suites with parameter shadowing and tag organization.")
                    })
                    .paragraph(|p| {
                        p.link(
                            &format!("{root}system/generation/rust/"),
                            "more \u{2192}",
                        )
                    })
                })
            })
            .rule()
            .section("Source", |s| {
                s.paragraph(|p| {
                    p.text("Deploys generated files to the workspace via a manifest-driven copy. Supports verification mode to detect drift between generated output and committed source.")
                })
                .shell("bazel run //:generate.documentation")
            })
            .rule()
            .aside(|a| {
                a.italic("These docs were generated with this framework \u{2014} authored as ")
                    .code(".document.rs")
                    .italic(" files and rendered to HTML by the web generator.")
            })
    })
}
