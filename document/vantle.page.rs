use body::Chain;
use extraction::Query;
use navigation::Composition;

pub fn page(root: &str) -> page::Result {
    navigation::layout("Vantle", &index::readme(root), root, |c| {
        c.title("Vantle")
            .anchor("https://github.com/Vantle/Vantle", |a| {
                a.class(class::subtitle())
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
                        .paragraph(|p| p.link(&index::molten::readme(root).href, "more \u{2192}"))
                })
                .subsection("Generation", |ss| {
                    ss.aside(|a| a.italic("Code generation framework for Rust"))
                        .paragraph(|p| {
                            p.text("Generate test suites from templates and JSON case definitions. The ")
                                .bold("autotest")
                                .text(" system eliminates boilerplate while enabling data-driven testing with parameter shadowing and tag organization.")
                        })
                        .paragraph(|p| p.link(&index::generation::generation(root).href, "more \u{2192}"))
                })
                .subsection("Observation", |ss| {
                    ss.aside(|a| a.italic("Trace streaming and recording"))
                        .paragraph(|p| {
                            p.text("Stream traces peer-to-peer without a central server. The ")
                                .code("#[trace]")
                                .text(" macro instruments functions with channel-based filtering for selective observation to files or remote peers.")
                        })
                        .paragraph(|p| p.link(&index::observation::observation(root).href, "more \u{2192}"))
                })
                .subsection("Spatialize", |ss| {
                    ss.aside(|a| a.italic("GPU rendering infrastructure"))
                        .paragraph(|p| {
                            p.text("Render with wgpu using assembler-pattern context creation and frame-based draw submission. Golden ratio scaling utilities ensure harmonious visual proportions throughout.")
                        })
                        .paragraph(|p| p.link(&index::spatialize::spatialize(root).href, "more \u{2192}"))
                })
            })
            .rule()
            .section("Quick Start", |s| {
                s.subsection("Requirements", |ss| {
                    ss.paragraph(|p| {
                        p.link("https://bazel.build/", "Bazel").text(&format!(" \u{2265} {}", module_bazel_version::EXTRACTIONS.one().content.trim_start_matches(">=")))
                    })
                })
                .subsection("Build", |ss| ss.shell("bazel build //..."))
                .subsection("Test", |ss| ss.shell("bazel test //..."))
            })
    })
}
