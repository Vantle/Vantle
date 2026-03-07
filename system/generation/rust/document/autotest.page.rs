use body::Chain;
use extraction::Query;
use language::Language;
use style::Composition;

pub fn page(root: &str) -> page::Result {
    style::layout("Autotest", "autotest", "autotest", root, |c| {
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
                .extract(particle_cases_json::EXTRACTIONS.one())
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
                s.subsection("rust_autotest_template", |ss| {
                    ss.paragraph(|p| {
                        p.text("Compiles the template as a ")
                            .code("rust_library")
                            .text(" for IDE support. Automatically adds ")
                            .code("-A dead_code")
                            .text(". Shared by both function and performance modules.")
                    })
                    .extract(template_document::EXTRACTIONS.one())
                    .table(|t| {
                        t.header(["Parameter", "Description"])
                            .describe("src", "Template source file")
                            .describe("deps", "Template dependencies")
                    })
                })
            })
            .rule()
            .section("Modules", |s| {
                s.subsection("Function", |ss| {
                    ss.aside(|a| {
                        a.italic("Generates exhaustive test suites from template functions and JSON cases")
                    })
                    .paragraph(|p| {
                        p.text("Each template function is expanded into individual test cases with parameter shadowing, tag filtering, and schema validation.")
                    })
                    .paragraph(|p| {
                        p.link(
                            &format!("{root}system/generation/rust/function.html"),
                            "more \u{2192}",
                        )
                    })
                })
                .subsection("Performance", |ss| {
                    ss.aside(|a| {
                        a.italic("Regression-aware performance testing with statistical curve fitting")
                    })
                    .paragraph(|p| {
                        p.text("Measures execution time across scaling inputs, fits complexity curves, and enforces bounds on both wall time and R\u{00b2} determination.")
                    })
                    .paragraph(|p| {
                        p.link(
                            &format!("{root}system/generation/rust/performance.html"),
                            "more \u{2192}",
                        )
                    })
                })
            })
    })
}
