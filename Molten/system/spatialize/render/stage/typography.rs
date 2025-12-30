use std::sync::Arc;

use dimension::Viewport;
use error::{Error, Result};
use glyphon::fontdb::Source;
use glyphon::{
    Attrs, Buffer, Cache, Family, FontSystem, Metrics, Resolution, Shaping, SwashCache, TextArea,
    TextAtlas, TextBounds, TextRenderer, Viewport as GlyphonViewport,
};
use graph::Graph;
use matrix::Matrix;
use primitive::Label;
use proportion::scale;
use stage::Stage;
use tag::Tag;

pub struct Assembler<'a> {
    font: Option<&'a str>,
    device: Option<&'a wgpu::Device>,
    queue: Option<&'a wgpu::Queue>,
    format: Option<wgpu::TextureFormat>,
}

impl<'a> Assembler<'a> {
    #[must_use]
    pub fn new() -> Self {
        Self {
            font: None,
            device: None,
            queue: None,
            format: None,
        }
    }

    #[must_use]
    pub fn font(mut self, path: &'a str) -> Self {
        self.font = Some(path);
        self
    }

    #[must_use]
    pub fn device(mut self, device: &'a wgpu::Device) -> Self {
        self.device = Some(device);
        self
    }

    #[must_use]
    pub fn queue(mut self, queue: &'a wgpu::Queue) -> Self {
        self.queue = Some(queue);
        self
    }

    #[must_use]
    pub fn format(mut self, format: wgpu::TextureFormat) -> Self {
        self.format = Some(format);
        self
    }

    pub fn assemble(self) -> Result<Typography> {
        let font = self.font.ok_or_else(|| Error::Configuration {
            field: "font".into(),
        })?;
        let device = self.device.ok_or_else(|| Error::Configuration {
            field: "device".into(),
        })?;
        let queue = self.queue.ok_or_else(|| Error::Configuration {
            field: "queue".into(),
        })?;
        let format = self.format.ok_or_else(|| Error::Configuration {
            field: "format".into(),
        })?;

        let bytes = std::fs::read(font).map_err(|source| Error::Font { source })?;

        let mut system = FontSystem::new();
        system
            .db_mut()
            .load_font_source(Source::Binary(Arc::new(bytes)));

        let swash = SwashCache::new();
        let cache = Cache::new(device);
        let viewport = GlyphonViewport::new(device, &cache);
        let mut atlas = TextAtlas::new(device, queue, &cache, format);
        let depth = Some(wgpu::DepthStencilState {
            format: wgpu::TextureFormat::Depth32Float,
            depth_write_enabled: false,
            depth_compare: wgpu::CompareFunction::LessEqual,
            stencil: wgpu::StencilState::default(),
            bias: wgpu::DepthBiasState::default(),
        });
        let renderer =
            TextRenderer::new(&mut atlas, device, wgpu::MultisampleState::default(), depth);

        Ok(Typography {
            system,
            swash,
            atlas,
            renderer,
            viewport,
            buffers: Vec::new(),
            projected: Vec::new(),
            labels: Vec::new(),
            view: Matrix::identity(),
            projection: Matrix::identity(),
            extent: Viewport::new(1, 1),
        })
    }
}

impl Default for Assembler<'_> {
    fn default() -> Self {
        Self::new()
    }
}

pub struct Typography {
    system: FontSystem,
    swash: SwashCache,
    atlas: TextAtlas,
    renderer: TextRenderer,
    viewport: GlyphonViewport,
    buffers: Vec<Buffer>,
    projected: Vec<Option<(f32, f32)>>,
    labels: Vec<Label>,
    view: Matrix,
    projection: Matrix,
    extent: Viewport,
}

impl Typography {
    pub fn measure(&mut self, text: &str, size: f32) -> f32 {
        let leading = size * scale().compute();
        let metrics = Metrics::new(size, leading);
        let mut buffer = Buffer::new(&mut self.system, metrics);
        buffer.set_size(&mut self.system, Some(10000.0), None);
        let attrs = Attrs::new().family(Family::SansSerif);
        buffer.set_text(&mut self.system, text, &attrs, Shaping::Advanced, None);
        buffer.shape_until_scroll(&mut self.system, false);
        buffer
            .layout_runs()
            .map(|run| run.line_w)
            .fold(0.0_f32, f32::max)
    }

    pub fn resize(&mut self, queue: &wgpu::Queue, width: u32, height: u32) {
        self.viewport.update(queue, Resolution { width, height });
    }

    pub fn feed(&mut self, labels: Vec<Label>, view: Matrix, projection: Matrix, extent: Viewport) {
        self.labels = labels;
        self.view = view;
        self.projection = projection;
        self.extent = extent;
    }

    pub fn prepare(&mut self, device: &wgpu::Device, queue: &wgpu::Queue) -> Result<()> {
        self.buffers.clear();
        self.projected.clear();

        for label in &self.labels {
            let screen = Self::project(label, self.view, self.projection, self.extent);
            self.projected.push(screen);

            if screen.is_some() {
                let leading = label.size * scale().compute();
                let metrics = Metrics::new(label.size, leading);
                let mut buffer = Buffer::new(&mut self.system, metrics);
                buffer.set_size(&mut self.system, Some(1000.0), Some(100.0));
                let attrs = Attrs::new().family(Family::SansSerif);
                buffer.set_text(
                    &mut self.system,
                    &label.content,
                    &attrs,
                    Shaping::Advanced,
                    None,
                );
                buffer.shape_until_scroll(&mut self.system, false);
                self.buffers.push(buffer);
            }
        }

        let visible = self
            .labels
            .iter()
            .zip(self.projected.iter())
            .filter_map(|(label, screen)| screen.map(|s| (label, s)));

        let areas = self
            .buffers
            .iter()
            .zip(visible)
            .map(|(buffer, (label, (x, y)))| TextArea {
                buffer,
                left: x,
                top: y,
                scale: 1.0,
                bounds: TextBounds::default(),
                default_color: glyphon::Color::rgba(
                    #[expect(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
                    {
                        (label.color.r * 255.0) as u8
                    },
                    #[expect(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
                    {
                        (label.color.g * 255.0) as u8
                    },
                    #[expect(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
                    {
                        (label.color.b * 255.0) as u8
                    },
                    #[expect(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
                    {
                        (label.color.a * 255.0) as u8
                    },
                ),
                custom_glyphs: &[],
            });

        let _ = self.renderer.prepare(
            device,
            queue,
            &mut self.system,
            &mut self.atlas,
            &self.viewport,
            areas,
            &mut self.swash,
        );
        Ok(())
    }

    pub fn draw(
        &self,
        encoder: &mut wgpu::CommandEncoder,
        surface: &wgpu::TextureView,
        graph: &Graph,
    ) -> Result<()> {
        let depth = graph.texture(Tag::new("depth"))?;

        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("typography::pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: surface,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Load,
                    store: wgpu::StoreOp::Store,
                },
                depth_slice: None,
            })],
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                view: depth,
                depth_ops: Some(wgpu::Operations {
                    load: wgpu::LoadOp::Load,
                    store: wgpu::StoreOp::Store,
                }),
                stencil_ops: None,
            }),
            timestamp_writes: None,
            occlusion_query_set: None,
            multiview_mask: None,
        });
        self.renderer
            .render(&self.atlas, &self.viewport, &mut pass)
            .map_err(|_| Error::Text)
    }

    fn project(
        label: &Label,
        view: Matrix,
        projection: Matrix,
        viewport: Viewport,
    ) -> Option<(f32, f32)> {
        let mvp = projection.multiply(&view);
        let (x, y, z, w) = mvp.transform(label.position);

        if w <= 0.0 {
            return None;
        }

        let ndc = (x / w, y / w, z / w);

        if ndc.2 < -1.0 || ndc.2 > 1.0 {
            return None;
        }

        let (width, height) = viewport.dimensions();
        let screen = ((ndc.0 + 1.0) * 0.5 * width, (1.0 - ndc.1) * 0.5 * height);

        Some(screen)
    }
}

impl Stage for Typography {
    fn prepare(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        _graph: &Graph,
    ) -> Result<()> {
        self.prepare(device, queue)
    }

    fn draw(
        &self,
        encoder: &mut wgpu::CommandEncoder,
        surface: &wgpu::TextureView,
        graph: &Graph,
    ) -> Result<()> {
        self.draw(encoder, surface, graph)
    }
}
