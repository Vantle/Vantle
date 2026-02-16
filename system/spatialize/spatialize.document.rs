use element::{Element, Language};
use span::Fragment;
use vantle::Composition;

fn code(value: &str) -> Element {
    Element::Span(vec![Fragment::Code(value.into())])
}

fn text(value: &str) -> Element {
    Element::Text(value.into())
}

fn main() -> miette::Result<()> {
    let arguments = render::Arguments::parse();
    vantle::page(&arguments, "Spatialize", "vantle", "spatialize", |c| {
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
                    .literal("use render::{Assembler, Context};\n\nlet context = Assembler::new()\n    .surface(surface)\n    .adapter(adapter)\n    .size(width, height)\n    .assemble()\n    .await?;", Language::Rust)
                    .table(|t| {
                        t.header(["Field", "Description"])
                            .markup([code("surface"), text("wgpu surface for presentation")])
                            .markup([code("adapter"), text("wgpu adapter for device creation")])
                            .markup([code("size"), text("Initial viewport dimensions")])
                    })
                })
                .subsection("Pipelines", |ss| {
                    ss.paragraph(|p| {
                        p.text("Build GPU pipelines with the raster and compute assemblers:")
                    })
                    .literal("use raster::Raster;\n\nlet pipeline = Raster::assembler()\n    .shader(\"path/to/pipeline.wgsl\")\n    .vertex(Vertex::layout())\n    .bind(0, Binding::uniform(wgpu::ShaderStages::VERTEX))\n    .target(format, Some(wgpu::BlendState::ALPHA_BLENDING))\n    .assemble(device)?;", Language::Rust)
                })
            })
            .rule()
            .section("Structure", |s| {
                s.element("pre", |p| {
                    p.element("code", |c| {
                        c.text("system/spatialize/\n  render/           GPU pipeline and frame management\n  interact/         Input and collision systems\n  proportion.rs     Golden ratio utilities")
                    })
                })
            })
    })
}
