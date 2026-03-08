use body::Chain;
use element::Element;
use extraction::Query;
use navigation::Composition;
use span::Fragment;

pub fn page(root: &str) -> page::Result {
    navigation::layout(
        "Spatialize",
        &index::molten::spatialize::spatialize(root),
        root,
        |c| {
            c.title("Spatialize")
            .subtitle("Interactive hypergraph visualization for Molten")
            .rule()
            .paragraph(|p| {
                p.text("This document describes ")
                    .bold("Molten Spatialize")
                    .text(", an interactive visualization system for hypergraph exploration.")
            })
            .rule()
            .section("Invoke", |s| {
                s.paragraph(|p| p.text("Run the spatialize visualization:"))
                    .extract(command_spatialize::EXTRACTIONS.one())
                    .paragraph(|p| {
                        p.text("This opens an interactive window displaying hypergraph state with real-time layout simulation.")
                    })
            })
            .rule()
            .section("Panes", |s| {
                s.paragraph(|p| p.text("Toggle between visualization modes:"))
                    .table(|t| {
                        t.header(["Pane", "Description", "Key"])
                            .markup([
                                Element::Text("Relation".into()),
                                Element::Text("Edge and node relationships".into()),
                                Element::Span(vec![Fragment::Code("Tab".into())]),
                            ])
                            .markup([
                                Element::Text("Inference".into()),
                                Element::Text("Derivation and inference paths".into()),
                                Element::Span(vec![Fragment::Code("Tab".into())]),
                            ])
                    })
            })
            .rule()
            .section("Controls", |s| {
                s.subsection("Navigation", |ss| {
                    ss.table(|t| {
                        t.header(["Action", "Control"])
                            .row(["Pan", "Left click + drag"])
                            .markup([
                                Element::Text("Rotate".into()),
                                Element::Text("Middle click + drag / Control + drag".into()),
                            ])
                            .row(["Zoom", "Scroll wheel / pinch"])
                            .row(["Select", "Right click"])
                    })
                })
                .subsection("View", |ss| {
                    ss.table(|t| {
                        t.header(["Action", "Control"])
                            .markup([
                                Element::Text("Toggle pane".into()),
                                Element::Span(vec![Fragment::Code("Tab".into())]),
                            ])
                            .markup([
                                Element::Text("Relation pane".into()),
                                Element::Span(vec![Fragment::Code("R".into())]),
                            ])
                            .markup([
                                Element::Text("Inference pane".into()),
                                Element::Span(vec![Fragment::Code("I".into())]),
                            ])
                            .markup([
                                Element::Text("Deselect".into()),
                                Element::Span(vec![Fragment::Code("Escape".into())]),
                            ])
                    })
                })
            })
            .rule()
            .section("Layout", |s| {
                s.paragraph(|p| {
                    p.text("Force-directed layout simulation positions nodes and edges automatically. The simulation uses:")
                })
                .list(|ul| {
                    ul.plain("Repulsion between nodes")
                        .plain("Attraction along edges")
                        .plain("Boundary constraints")
                })
            })
        },
    )
}
