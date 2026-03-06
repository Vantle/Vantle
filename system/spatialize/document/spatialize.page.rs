use body::Chain;
use language::Language;
use style::Composition;

pub fn page(root: &str) -> page::Result {
    style::layout("Spatialize", "vantle", "spatialize", root, |c| {
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
                    .code("use render::{Assembler, Context};\n\nlet context = Assembler::new()\n    .surface(surface)\n    .adapter(adapter)\n    .size(width, height)\n    .assemble()\n    .await?;", Language::Rust)
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
                    .code("use raster::Raster;\n\nlet pipeline = Raster::assembler()\n    .shader(\"path/to/pipeline.wgsl\")\n    .vertex(Vertex::layout())\n    .bind(0, Binding::uniform(wgpu::ShaderStages::VERTEX))\n    .target(format, Some(wgpu::BlendState::ALPHA_BLENDING))\n    .assemble(device)?;", Language::Rust)
                })
            })
    })
}
