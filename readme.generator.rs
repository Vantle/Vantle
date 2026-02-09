use vantle::Composition;

fn main() -> miette::Result<()> {
    let arguments = render::Arguments::parse();
    vantle::page(&arguments, "Vantle", |c| {
        c.image("resource/logo.png", "Vantle")
            .title("Vantle")
            .subtitle("Platform for everything")
            .navigation(|n| {
                n.link("Info.html", "Info")
                    .link("Notice.html", "Notice")
                    .link("Module.html", "Module")
                    .link("Molten/Readme.html", "Molten")
                    .link("system/generation/Readme.html", "Generation")
                    .link("system/observation/Readme.html", "Observation")
                    .link("system/spatialize/Readme.html", "Spatialize")
                    .link("License.html", "License")
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
                        .paragraph(|p| p.link("Molten/Readme.html", "more →"))
                })
                .subsection("Generation", |ss| {
                    ss.aside(|a| a.italic("Code generation framework for Rust"))
                        .paragraph(|p| {
                            p.text("Generate test suites from templates and JSON case definitions. The ")
                                .bold("autotest")
                                .text(" system eliminates boilerplate while enabling data-driven testing with parameter shadowing and tag organization.")
                        })
                        .paragraph(|p| p.link("system/generation/Readme.html", "more →"))
                })
                .subsection("Observation", |ss| {
                    ss.aside(|a| a.italic("Trace streaming and recording"))
                        .paragraph(|p| {
                            p.text("Stream traces peer-to-peer without a central server. The ")
                                .code("#[trace]")
                                .text(" macro instruments functions with channel-based filtering for selective observation to files or remote peers.")
                        })
                        .paragraph(|p| p.link("system/observation/Readme.html", "more →"))
                })
                .subsection("Spatialize", |ss| {
                    ss.aside(|a| a.italic("GPU rendering infrastructure"))
                        .paragraph(|p| {
                            p.text("Render with wgpu using assembler-pattern context creation and frame-based draw submission. Golden ratio scaling utilities ensure harmonious visual proportions throughout.")
                        })
                        .paragraph(|p| p.link("system/spatialize/Readme.html", "more →"))
                })
            })
            .rule()
            .section("Quick Start", |s| {
                s.subsection("Requirements", |ss| {
                    ss.paragraph(|p| {
                        p.link("https://bazel.build/", "Bazel").text(" >= 9.0.0")
                    })
                })
                .subsection("Build", |ss| ss.shell("bazel build //..."))
                .subsection("Test", |ss| ss.shell("bazel test //..."))
            })
    })
}
