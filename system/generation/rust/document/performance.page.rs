use element::Element;
use extraction::Query;

#[must_use]
pub fn page(root: &str) -> page::Page {
    navigation::layout(
        "Performance",
        &index::generation::autotest::performance(root),
        root,
        |c| {
            c.title("Performance")
            .subtitle("Regression-aware performance testing")
            .rule()
            .paragraph(|p| {
                p.text("Extends Autotest with statistical regression analysis. Define performance cases alongside functional cases, and the framework measures execution time, fits complexity curves, and enforces bounds.")
            })
            .rule()
            .section("Template", |s| {
                s.paragraph(|p| {
                    p.text("Performance templates follow the same ")
                        .code(".template.rs")
                        .text(" convention. The function parameter drives the scaling dimension:")
                })
                .extract(sort_document::EXTRACTIONS.one())
            })
            .rule()
            .section("Cases", |s| {
                s.paragraph(|p| {
                    p.text("Define scaling inputs in ")
                        .code("cases.json")
                        .text(". Each case varies the measured parameter across a range of sizes:")
                })
                .extract(sort_cases_json::EXTRACTIONS.one())
            })
            .rule()
            .section("Specification", |s| {
                s.paragraph(|p| {
                    p.text("The specification in ")
                        .code("performance.cases.json")
                        .text(" configures measurement and validation:")
                })
                .extract(specification_json::EXTRACTIONS.one())
                .table(|t| {
                    t.header(["Field", "Description"])
                        .describe("select", "Tag filter selecting which cases to benchmark")
                        .describe("measure", "Maps parameter names to scaling dimensions")
                        .describe("sampling", "Iteration count and warmup rounds per case")
                        .describe("bounds", "Structure assertions: polynomial term ordering with confidence thresholds")
                })
            })
            .rule()
            .section("Macro", |s| {
                s.section("rust_autotest_performance", |ss| {
                    ss.extract(performance_document::EXTRACTIONS.one())
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
                                .describe("specification", "Performance specification JSON")
                                .describe("size", "Test size classification")
                                .describe("deps", "Custom dependencies beyond defaults")
                        })
                })
            })
            .rule()
            .section("Execution", |s| {
                s.paragraph(|p| {
                    p.text("The performance report includes timing measurements, regression coefficients, and structural validation results:")
                })
                .extract(execution_schema::EXTRACTIONS.one())
            })
            .rule()
            .section("Features", |s| {
                s.list(|ul| {
                    ul.feature(
                        "Regression fitting",
                        ": Fits timing data to complexity curves",
                    )
                    .feature(
                        "Warmup",
                        ": Configurable warmup iterations before measurement",
                    )
                    .feature("Bounds", ": Structure assertions enforce polynomial term ordering")
                    .feature(
                        "R\u{00b2} determination",
                        ": Statistical goodness-of-fit for the regression model",
                    )
                })
            })
        },
    )
}
