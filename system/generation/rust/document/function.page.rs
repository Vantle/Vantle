use body::Chain;
use element::Element;
use extraction::Query;
use span::Fragment;
use style::Composition;

pub fn page(root: &str) -> page::Result {
    style::layout("Function", "autotest", "function", root, |c| {
        c.title("Function")
            .subtitle("Functional test generation")
            .rule()
            .paragraph(|p| {
                p.text("Generates and runs Rust test functions from templates and JSON case definitions. Standard Vantle dependencies are auto-included; only add deps beyond the defaults.")
            })
            .rule()
            .section("Macro", |s| {
                s.subsection("rust_autotest_function", |ss| {
                    ss.extract(function_document::EXTRACTIONS.one())
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
