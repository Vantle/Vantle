use element::Language;
use style::Composition;

fn main() -> miette::Result<()> {
    html::execute(|arguments| {
        let root = arguments.root();
        style::page(arguments, "Observation", "vantle", "observation", |c| {
            c.title("Observation")
            .subtitle("Trace streaming and recording for Vantle")
            .rule()
            .paragraph(|p| {
                p.text("This document describes the ")
                    .bold("Observation")
                    .text(" system, a peer-to-peer trace streaming framework for Vantle.")
            })
            .rule()
            .section("Architecture", |s| {
                s.paragraph(|p| {
                    p.text(
                        "Observation uses a peer-to-peer model with no central server. Applications stream traces directly to:",
                    )
                })
                .element("ul", |ul| {
                    ul.element("li", |li| {
                        li.span(|s| {
                            s.bold("Files")
                                .text(": Local recording via ")
                                .code("log://")
                                .text(" and ")
                                .code("chrome://")
                                .text(" URIs")
                        })
                    })
                    .element("li", |li| {
                        li.span(|s| {
                            s.bold("Peers")
                                .text(": Remote streaming via ")
                                .code("grpc://")
                                .text(" URIs")
                        })
                    })
                })
                .paragraph(|p| {
                    p.text("Each application decides where to send its traces. See ")
                        .link(&format!("{root}Molten/#forge"), "Forge")
                        .text(" for an example of configuring trace destinations.")
                })
            })
            .rule()
            .section("Trace", |s| {
                s.paragraph(|p| {
                    p.text("The ")
                        .code("#[trace]")
                        .text(" macro instruments functions for observation.")
                })
                .subsection("Usage", |ss| {
                    ss.literal(
                        "#[trace(channels = [core])]\nfn process() {\n    evaluate();\n}",
                        Language::Rust,
                    )
                })
                .subsection("Channels", |ss| {
                    ss.paragraph(|p| {
                        p.text("Channels filter which spans to emit. Common channels include:")
                    })
                    .element("ul", |ul| {
                        ul.element("li", |li| {
                            li.span(|s| s.code("core").text(": Core runtime operations"))
                        })
                        .element("li", |li| {
                            li.span(|s| s.code("document").text(": Documentation generation"))
                        })
                        .element("li", |li| {
                            li.span(|s| s.code("hypergraph").text(": Hypergraph evaluation"))
                        })
                        .element("li", |li| {
                            li.span(|s| s.code("matching").text(": Pattern matching"))
                        })
                        .element("li", |li| {
                            li.span(|s| s.code("query").text(": Graph queries"))
                        })
                    })
                })
            })
            .rule()
            .section("Structure", |s| {
                s.element("pre", |p| {
                    p.element("code", |c| {
                        c.text("component/observation/     Streaming layer and span types\nsystem/observation/        Trace initialization and encoding")
                    })
                })
            })
        })
    })
}
