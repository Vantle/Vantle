use body::Chain;
use element::Element;
use extraction::Query;
use language::Language;
use span::Fragment;
use style::Composition;

pub fn page(root: &str) -> page::Result {
    style::layout("Autotest", "generation", "autotest", root, |c| {
        c.title("Autotest")
            .subtitle("JSON-driven test generation for Rust")
            .rule()
            .paragraph(|p| {
                p.text("Autotest eliminates test boilerplate by generating Rust test functions from templates and JSON case definitions. Write the logic once, define test data declaratively, and let the framework produce exhaustive test suites.")
            })
            .rule()
            .section("Template", |s| {
                s.paragraph(|p| {
                    p.text("Write functions in a ")
                        .code(".template.rs")
                        .text(" file. Each function becomes a test generator:")
                })
                .extract(disjoint::EXTRACTIONS.one())
            })
            .rule()
            .section("Cases", |s| {
                s.paragraph(|p| {
                    p.text("Define test data in ")
                        .code("cases.json")
                        .text(". Each function specifies default parameters, expected returns, and individual cases that can override defaults:")
                })
                .extract(cases_document::EXTRACTIONS.one())
            })
            .rule()
            .section("Execution", |s| {
                s.paragraph(|p| {
                    p.text("Each test run produces a ")
                        .code("cases.execution.json")
                        .text(" report. Functions are grouped with their cases, and mismatches surface as ")
                        .code("unexpected")
                        .text(" values:")
                })
                .code("{\n  \"source\": {\n    \"file\": \"particle.template.rs\",\n    \"cases\": \"cases.json\"\n  },\n  \"functions\": [\n    {\n      \"function\": \"disjoint\",\n      \"tags\": [\"complete\"],\n      \"cases\": [\n        {\n          \"parameters\": { \"candidate\": [[\"a\", 1]], \"basis\": [[\"b\", 2]] },\n          \"returns\": { \"()\": [[\"a\", 1]] },\n          \"unexpected\": null\n        }\n      ]\n    }\n  ]\n}", Language::Json)
            })
            .rule()
            .section("Macro", |s| {
                s.paragraph(|p| {
                    p.text("Two Starlark macros work as a pair. The template compiles as a ")
                        .code("rust_library")
                        .text(" for IDE support. The generator reads the template and cases, then produces a ")
                        .code(".generated.rs")
                        .text(" test file that runs via ")
                        .code("rust_test")
                        .text(".")
                })
                .code("load(\"@vantle//component/generation/starlark:defs.bzl\", \"rust_autotest\", \"rust_autotest_template\")\n\nrust_autotest_template(\n    name = \"template\",\n    src = \"function.template.rs\",\n    deps = [\"//Molten/component/graph/state/particle:module\"],\n)\n\nrust_autotest(\n    name = \"function\",\n    template = \":template\",\n    cases = \":cases.json\",\n    deps = [\"//Molten/component/graph/state/particle:module\"],\n)", Language::Starlark)
                .subsection("rust_autotest_template", |ss| {
                    ss.paragraph(|p| {
                        p.text("Validates template compilation and enables IDE support. Automatically adds ")
                            .code("-A dead_code")
                            .text(".")
                    })
                    .table(|t| {
                        t.header(["Parameter", "Description"])
                            .describe("src", "Template source file")
                            .describe("deps", "Template dependencies")
                    })
                })
                .subsection("rust_autotest", |ss| {
                    ss.paragraph(|p| {
                        p.text("Generates and runs test functions. Standard Vantle dependencies are auto-included; only add deps beyond the defaults.")
                    })
                    .table(|t| {
                        t.header(["Parameter", "Description"])
                            .markup([
                                Element::Span(vec![Fragment::Code("template".into())]),
                                Element::Span(vec![
                                    Fragment::Text("Template target from ".into()),
                                    Fragment::Code("rust_autotest_template".into()),
                                ]),
                            ])
                            .describe("cases", "JSON test case definitions")
                            .describe("deps", "Custom dependencies beyond defaults")
                    })
                })
            })
            .rule()
            .section("Features", |s| {
                s.list(|ul| {
                    ul.feature(
                        "Parameter shadowing",
                        ": Function-level defaults with case-level overrides",
                    )
                    .feature("Tag organization", ": Filter tests by tags")
                    .feature(
                        "Schema validation",
                        ": Parameters match function signatures",
                    )
                    .feature(
                        "Rich diagnostics",
                        ": Error reporting via miette with source locations",
                    )
                })
            })
    })
}
