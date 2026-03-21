use element::Element;
use extraction::Query;

#[must_use]
pub fn page(root: &str) -> page::Page {
    navigation::layout(
        "Function",
        &index::generation::autotest::function(root),
        root,
        |c| {
            c.title("Function")
            .subtitle("Functional test generation")
            .rule()
            .paragraph(|p| {
                p.text("Generates and runs Rust test functions from templates and JSON case definitions. Standard Vantle dependencies are auto-included; only add deps beyond the defaults.")
            })
            .rule()
            .chapter("Macro", |s| {
                s.chapter("rust_autotest_function", |ss| {
                    ss.extract(function_document::EXTRACTIONS.one())
                        .table(|t| {
                            t.header(["Parameter", "Description"])
                                .markup([
                                    Element::Tag {
                                        name: "code".into(),
                                        attributes: Vec::new(),
                                        children: vec![Element::Text("template".into())],
                                    },
                                    Element::Tag {
                                        name: "span".into(),
                                        attributes: Vec::new(),
                                        children: vec![
                                            Element::Text("Template target from ".into()),
                                            Element::Tag {
                                                name: "code".into(),
                                                attributes: Vec::new(),
                                                children: vec![Element::Text("rust_autotest_template".into())],
                                            },
                                        ],
                                    },
                                ])
                                .describe("cases", "JSON test case definitions")
                                .describe("deps", "Custom dependencies beyond defaults")
                        })
                })
            })
            .rule()
            .chapter("Features", |s| {
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
        },
    )
}
