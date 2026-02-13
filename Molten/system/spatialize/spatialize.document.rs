use vantle::Composition;

fn main() -> miette::Result<()> {
    let arguments = render::Arguments::parse();
    vantle::page(
        &arguments,
        "Molten Spatialize",
        "molten",
        "spatialize",
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
                    .shell("bazel run //Molten/system/spatialize:command")
                    .paragraph(|p| {
                        p.text("This opens an interactive window displaying hypergraph state with real-time layout simulation.")
                    })
            })
            .rule()
            .section("Panes", |s| {
                s.paragraph(|p| p.text("Toggle between visualization modes:"))
                    .table(|t| {
                        t.header(["Pane", "Description", "Key"])
                            .row(["Relation", "Edge and node relationships", "Tab"])
                            .row(["Inference", "Derivation and inference paths", "Tab"])
                    })
            })
            .rule()
            .section("Controls", |s| {
                s.subsection("Navigation", |ss| {
                    ss.table(|t| {
                        t.header(["Action", "Control"])
                            .row(["Pan", "Left click + drag"])
                            .row(["Rotate", "Middle click + drag / Control + drag"])
                            .row(["Zoom", "Scroll wheel / pinch"])
                            .row(["Select", "Right click"])
                    })
                })
                .subsection("View", |ss| {
                    ss.table(|t| {
                        t.header(["Action", "Control"])
                            .row(["Toggle pane", "Tab"])
                            .row(["Relation pane", "R key"])
                            .row(["Inference pane", "I key"])
                            .row(["Deselect", "Escape"])
                    })
                })
            })
            .rule()
            .section("Layout", |s| {
                s.paragraph(|p| {
                    p.text("Force-directed layout simulation positions nodes and edges automatically. The simulation uses:")
                })
                .element("ul", |ul| {
                    ul.element("li", |li| li.text("Repulsion between nodes"))
                        .element("li", |li| li.text("Attraction along edges"))
                        .element("li", |li| li.text("Boundary constraints"))
                })
            })
            .rule()
            .section("Structure", |s| {
                s.element("pre", |p| {
                    p.element("code", |c| {
                        c.text("Molten/system/spatialize/\n  command.rs        Application entry point\n  pane.rs           Visualization pane modes\n  view.rs           View state and transformations\n  layout.rs         Force-directed simulation\n  scene.rs          Scene graph management\n  render.rs         Render submission\n  mouse.rs          Input state tracking\n  palette.rs        Color definitions")
                    })
                })
            })
        },
    )
}
