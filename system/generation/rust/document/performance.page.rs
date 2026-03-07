use body::Chain;
use element::Element;
use extraction::Query;
use span::Fragment;
use style::Composition;

pub fn page(root: &str) -> page::Result {
    style::layout("Performance", "autotest", "performance", root, |c| {
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
                        .markup([
                            Element::Span(vec![Fragment::Code("select".into())]),
                            Element::Span(vec![Fragment::Text(
                                "Tag filter selecting which cases to benchmark".into(),
                            )]),
                        ])
                        .markup([
                            Element::Span(vec![Fragment::Code("measure".into())]),
                            Element::Span(vec![Fragment::Text(
                                "Maps parameter names to scaling dimensions".into(),
                            )]),
                        ])
                        .markup([
                            Element::Span(vec![Fragment::Code("sampling".into())]),
                            Element::Span(vec![Fragment::Text(
                                "Iteration count and warmup rounds per case".into(),
                            )]),
                        ])
                        .markup([
                            Element::Span(vec![Fragment::Code("bounds".into())]),
                            Element::Span(vec![Fragment::Text(
                                "Constraints: time limits at specific inputs and R\u{00b2} determination thresholds".into(),
                            )]),
                        ])
                })
            })
            .rule()
            .section("Macro", |s| {
                s.subsection("rust_autotest_performance", |ss| {
                    ss.extract(performance_document::EXTRACTIONS.one())
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
                                .describe("specification", "Performance specification JSON")
                                .describe("size", "Test size classification")
                                .describe("deps", "Custom dependencies beyond defaults")
                        })
                })
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
                    .feature("Bounds", ": Time limits at specific input sizes")
                    .feature(
                        "R\u{00b2} determination",
                        ": Statistical confidence threshold for curve fit",
                    )
                })
            })
    })
}
