use body::Chain;
use extraction::Query;
use language::Language;
use style::Composition;

fn main() -> miette::Result<()> {
    html::execute(|arguments| {
        style::page(arguments, "Autotest", "generation", "autotest", |c| {
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
                    .element("table", |t| {
                        t.element("thead", |h| {
                            h.element("tr", |r| {
                                r.element("th", |c| c.text("Parameter"))
                                    .element("th", |c| c.text("Description"))
                            })
                        })
                        .element("tbody", |b| {
                            b.element("tr", |r| {
                                r.element("td", |c| c.element("code", |c| c.text("src")))
                                    .element("td", |c| {
                                        c.text("Template source file")
                                    })
                            })
                            .element("tr", |r| {
                                r.element("td", |c| c.element("code", |c| c.text("deps")))
                                    .element("td", |c| {
                                        c.text("Template dependencies")
                                    })
                            })
                        })
                    })
                })
                .subsection("rust_autotest", |ss| {
                    ss.paragraph(|p| {
                        p.text("Generates and runs test functions. Standard Vantle dependencies are auto-included; only add deps beyond the defaults.")
                    })
                    .element("table", |t| {
                        t.element("thead", |h| {
                            h.element("tr", |r| {
                                r.element("th", |c| c.text("Parameter"))
                                    .element("th", |c| c.text("Description"))
                            })
                        })
                        .element("tbody", |b| {
                            b.element("tr", |r| {
                                r.element("td", |c| {
                                    c.element("code", |c| c.text("template"))
                                })
                                .element("td", |c| {
                                    c.text("Template target from ")
                                        .element("code", |c| {
                                            c.text("rust_autotest_template")
                                        })
                                })
                            })
                            .element("tr", |r| {
                                r.element("td", |c| c.element("code", |c| c.text("cases")))
                                    .element("td", |c| {
                                        c.text("JSON test case definitions")
                                    })
                            })
                            .element("tr", |r| {
                                r.element("td", |c| c.element("code", |c| c.text("deps")))
                                    .element("td", |c| {
                                        c.text("Custom dependencies beyond defaults")
                                    })
                            })
                        })
                    })
                })
            })
            .rule()
            .section("Features", |s| {
                s.element("ul", |ul| {
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
                                .text(": Error reporting via miette with source locations")
                        })
                    })
                })
            })
        })
    })
}
