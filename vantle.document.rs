use style::Composition;

fn main() -> miette::Result<()> {
    let arguments = html::Arguments::parse();
    let root = arguments.root();
    style::page(&arguments, "Vantle", "vantle", "readme", |c| {
        c.title("Vantle")
            .element("a", |a| {
                a.class("subtitle")
                    .attribute("href", "https://github.com/Vantle/Vantle")
                    .text("Platform for everything")
            })
            .rule()
            .paragraph(|p| {
                p.bold("Vantle")
                    .text(" is a platform for software research and experimentation.")
            })
            .rule()
            .section("Features", |s| {
                s.subsection("Molten", |ss| {
                    ss.aside(|a| a.italic("Computational expression over hypergraphs"))
                        .paragraph(|p| {
                            p.text("An AI frontend language designed for continual learning algorithms. Build hypergraphs through polymorphic relations, enabling declarative computation with concepts, orthogonalities, and transformations evaluated with temporal semantics.")
                        })
                        .paragraph(|p| p.link(&format!("{root}Molten/"), "more \u{2192}"))
                })
                .subsection("Generation", |ss| {
                    ss.aside(|a| a.italic("Code generation framework for Rust"))
                        .paragraph(|p| {
                            p.text("Generate test suites from templates and JSON case definitions. The ")
                                .bold("autotest")
                                .text(" system eliminates boilerplate while enabling data-driven testing with parameter shadowing and tag organization.")
                        })
                        .paragraph(|p| p.link(&format!("{root}system/generation/"), "more \u{2192}"))
                })
                .subsection("Observation", |ss| {
                    ss.aside(|a| a.italic("Trace streaming and recording"))
                        .paragraph(|p| {
                            p.text("Stream traces peer-to-peer without a central server. The ")
                                .code("#[trace]")
                                .text(" macro instruments functions with channel-based filtering for selective observation to files or remote peers.")
                        })
                        .paragraph(|p| p.link(&format!("{root}system/observation/"), "more \u{2192}"))
                })
                .subsection("Spatialize", |ss| {
                    ss.aside(|a| a.italic("GPU rendering infrastructure"))
                        .paragraph(|p| {
                            p.text("Render with wgpu using assembler-pattern context creation and frame-based draw submission. Golden ratio scaling utilities ensure harmonious visual proportions throughout.")
                        })
                        .paragraph(|p| p.link(&format!("{root}system/spatialize/"), "more \u{2192}"))
                })
            })
            .rule()
            .section("Quick Start", |s| {
                s.subsection("Requirements", |ss| {
                    ss.paragraph(|p| {
                        p.link("https://bazel.build/", "Bazel").text(" \u{2265} 9.0.0")
                    })
                })
                .subsection("Build", |ss| ss.shell("bazel build //..."))
                .subsection("Test", |ss| ss.shell("bazel test //..."))
            })
    })
}
