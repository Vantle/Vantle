use body::Chain;
use extraction::Query;
use navigation::Composition;

pub fn page(root: &str) -> page::Result {
    navigation::layout(
        "Spatialize",
        &index::spatialize::spatialize(root),
        root,
        |c| {
            c.title("Spatialize")
                .subtitle("GPU rendering infrastructure for Vantle")
                .rule()
                .paragraph(|p| {
                    p.text("This document describes the ")
                        .bold("Spatialize")
                        .text(" system, a GPU rendering infrastructure built on wgpu.")
                })
                .rule()
                .section("Context", |s| {
                    s.paragraph(|p| {
                        p.text("The render context manages GPU resources and pipeline state.")
                    })
                    .subsection("Assembler", |ss| {
                        ss.paragraph(|p| {
                            p.text("Build a rendering context with the assembler pattern:")
                        })
                        .extract(launcher_assembler::EXTRACTIONS.one())
                        .table(|t| {
                            t.header(["Field", "Description"])
                                .describe("surface", "wgpu surface for presentation")
                                .describe("adapter", "wgpu adapter for device creation")
                                .describe("size", "Initial viewport dimensions")
                        })
                    })
                    .subsection("Pipelines", |ss| {
                        ss.paragraph(|p| {
                            p.text("Build GPU pipelines with the raster and compute assemblers:")
                        })
                        .extract(geometry_pipeline::EXTRACTIONS.one())
                    })
                })
        },
    )
}
