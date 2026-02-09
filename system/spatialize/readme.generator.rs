use vantle::Composition;

fn main() -> miette::Result<()> {
    let arguments = render::Arguments::parse();
    vantle::page(&arguments, "Spatialize", |c| {
        c.title("Spatialize")
            .subtitle("GPU rendering infrastructure for Vantle")
            .navigation(|n| {
                n.link("../../Readme.html", "Vantle")
                    .link("../../Module.html", "Module")
                    .link("../../Molten/system/spatialize/Readme.html", "Molten Spatialize")
                    .link("../../License.html", "License")
            })
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
                    .element("pre", |p| {
                        p.element("code", |c| {
                            c.class("language-rust").text("use render::{Assembler, Context};\n\nlet context = Assembler::new()\n    .surface(surface)\n    .adapter(adapter)\n    .size(width, height)\n    .assemble()\n    .await?;")
                        })
                    })
                    .table(|t| {
                        t.header(["Field", "Description"])
                            .row(["surface", "wgpu surface for presentation"])
                            .row(["adapter", "wgpu adapter for device creation"])
                            .row(["size", "Initial viewport dimensions"])
                    })
                })
                .subsection("Pipelines", |ss| {
                    ss.paragraph(|p| {
                        p.text("Build GPU pipelines with the raster and compute assemblers:")
                    })
                    .element("pre", |p| {
                        p.element("code", |c| {
                            c.class("language-rust").text("use raster::Raster;\n\nlet pipeline = Raster::assembler()\n    .shader(\"path/to/pipeline.wgsl\")\n    .vertex(Vertex::layout())\n    .bind(0, Binding::uniform(wgpu::ShaderStages::VERTEX))\n    .target(format, Some(wgpu::BlendState::ALPHA_BLENDING))\n    .assemble(device)?;")
                        })
                    })
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
