use element::Language;
use vantle::Composition;

fn main() -> miette::Result<()> {
    let arguments = render::Arguments::parse();
    vantle::page(&arguments, "Generation", "vantle", "generation", |c| {
        c.title("Generation")
            .subtitle("Code generation framework for Rust")
            .rule()
            .paragraph(|p| {
                p.text("This document describes the ")
                    .bold("Generation")
                    .text(" framework, a code generation system for Rust projects built on Bazel.")
            })
            .rule()
            .section("Autotest", |s| {
                s.paragraph(|p| {
                    p.text("Autotest is an implementation of the Generation framework that provides JSON-driven test generation, eliminating boilerplate and enabling data-driven testing.")
                })
                .subsection("Template", |ss| {
                    ss.paragraph(|p| {
                        p.text("Write functions in a ")
                            .code(".template.rs")
                            .text(" file:")
                    })
                    .literal("use component::graph::state::particle::Particle;\n\nfn disjoint(candidate: Particle<String>, basis: Particle<String>) -> Option<Particle<String>> {\n    candidate.disjoint(&basis).map(|_| candidate.clone())\n}", Language::Rust)
                })
                .subsection("Cases", |ss| {
                    ss.paragraph(|p| {
                        p.text("Define test data in ")
                            .code("cases.json")
                            .text(":")
                    })
                    .literal("{\n  \"functions\": [\n    {\n      \"function\": \"disjoint\",\n      \"tags\": [\"particle\", \"disjoint\"],\n      \"parameters\": {\n        \"candidate\": [[\"a\", 1]],\n        \"basis\": [[\"b\", 1]]\n      },\n      \"returns\": { \"()\": [[\"a\", 1]] },\n      \"cases\": [\n        {\n          \"tags\": [\"empty\"],\n          \"parameters\": { \"basis\": [] },\n          \"returns\": { \"()\": [[\"a\", 1]] }\n        }\n      ]\n    }\n  ]\n}", Language::Json)
                })
                .subsection("Build", |ss| {
                    ss.paragraph(|p| p.text("Use Bazel rules to generate and run tests:"))
                        .literal("rust_autotest_template(\n    name = \"template\",\n    src = \"function.template.rs\",\n    deps = [\"//Molten/component/graph/state/particle:module\"],\n)\n\nrust_autotest(\n    name = \"function\",\n    template = \":template\",\n    cases = \":cases.json\",\n    deps = [\"//Molten/component/graph/state/particle:module\"],\n)", Language::Starlark)
                })
                .subsection("Features", |ss| {
                    ss.element("ul", |ul| {
                        ul.element("li", |li| {
                            li.span(|s| {
                                s.bold("Parameter shadowing")
                                    .text(": Function-level defaults with case-level overrides")
                            })
                        })
                        .element("li", |li| {
                            li.span(|s| {
                                s.bold("Tag organization").text(": Filter tests by tags")
                            })
                        })
                        .element("li", |li| {
                            li.span(|s| {
                                s.bold("Schema validation")
                                    .text(": Parameters match function signatures")
                            })
                        })
                        .element("li", |li| {
                            li.span(|s| {
                                s.bold("Rich diagnostics")
                                    .text(": Error reporting via miette")
                            })
                        })
                    })
                })
            })
            .rule()
            .section("Structure", |s| {
                s.element("pre", |p| {
                    p.element("code", |c| {
                        c.text(
                            "component/generation/     Schema and types\nsystem/generation/        Generator binary",
                        )
                    })
                })
            })
    })
}
