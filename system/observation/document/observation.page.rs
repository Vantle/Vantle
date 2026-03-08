use body::Chain;
use extraction::Query;
use navigation::Composition;

pub fn page(root: &str) -> page::Result {
    navigation::layout(
        "Observation",
        &index::observation::observation(root),
        root,
        |c| {
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
                .unordered(|ul| {
                    ul.item(|li| {
                        li.span(|s| {
                            s.bold("Files")
                                .text(": Local recording via ")
                                .code("log://")
                                .text(" and ")
                                .code("chrome://")
                                .text(" URIs")
                        })
                    })
                    .item(|li| {
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
                        .link(&index::molten::readme(root).fragment("forge"), "Forge")
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
                    ss.extract(view_trace::EXTRACTIONS.one())
                })
                .subsection("Channels", |ss| {
                    ss.paragraph(|p| {
                        p.text("Channels filter which spans to emit. Common channels include:")
                    })
                    .list(|ul| {
                        ul.glossary("core", ": Core runtime operations")
                            .glossary("document", ": Documentation generation")
                            .glossary("hypergraph", ": Hypergraph evaluation")
                            .glossary("matching", ": Pattern matching")
                            .glossary("query", ": Graph queries")
                    })
                })
            })
        },
    )
}
