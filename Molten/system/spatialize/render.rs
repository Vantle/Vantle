pub use error;

use assemble::Async;
use color::Color;
use dimension::Viewport;
use error::{Error, Result};
use geometry::Geometry;
use gpu::Graph;
use observer::Observer;
use outline::Outline;
use primitive::Label;
use render::Context;
use resource::Scale;
use stage::Stage;
use tag::Tag;
use typography::Typography;

const FONT: &str = "Molten/resource/system/spatialize/glyph/font/inter.ttf";

pub struct Assembler {
    surface: Option<wgpu::Surface<'static>>,
    adapter: Option<wgpu::Adapter>,
    size: Option<(u32, u32)>,
}

impl Assembler {
    #[must_use]
    pub fn new() -> Self {
        Self {
            surface: None,
            adapter: None,
            size: None,
        }
    }

    #[must_use]
    pub fn surface(mut self, surface: wgpu::Surface<'static>) -> Self {
        self.surface = Some(surface);
        self
    }

    #[must_use]
    pub fn adapter(mut self, adapter: wgpu::Adapter) -> Self {
        self.adapter = Some(adapter);
        self
    }

    #[must_use]
    pub fn size(mut self, width: u32, height: u32) -> Self {
        self.size = Some((width, height));
        self
    }

    pub async fn assemble(self) -> Result<Renderer> {
        let surface = self.surface.ok_or(Error::Configuration {
            field: "surface".into(),
        })?;
        let adapter = self.adapter.ok_or(Error::Configuration {
            field: "adapter".into(),
        })?;
        let (width, height) = self.size.ok_or(Error::Configuration {
            field: "size".into(),
        })?;

        let context = render::Assembler::new()
            .surface(surface)
            .adapter(adapter)
            .size(width, height)
            .assemble()
            .await?;

        Renderer::new(context, width, height)
    }
}

impl Async for Assembler {
    type Output = Result<Renderer>;

    fn assemble(self) -> impl std::future::Future<Output = Self::Output> + Send {
        Self::assemble(self)
    }
}

impl Default for Assembler {
    fn default() -> Self {
        Self::new()
    }
}

pub struct Renderer {
    context: Context,
    graph: Graph,
    geometry: Geometry,
    outline: Outline,
    typography: Typography,
}

impl Renderer {
    fn new(context: Context, width: u32, height: u32) -> Result<Self> {
        let device = context.device();
        let queue = context.queue();
        let format = context.format();

        let geometry = geometry::Assembler::new()
            .device(device)
            .format(format)
            .assemble()?;

        let outline = outline::Assembler::new()
            .device(device)
            .format(format)
            .assemble()?;

        let typography = typography::Assembler::new()
            .font(FONT)
            .device(device)
            .queue(queue)
            .format(format)
            .assemble()?;

        let graph = Graph::assembler()
            .texture(Tag::new("mask"), wgpu::TextureFormat::Rg8Unorm, Scale::Full)
            .buffer(
                Tag::new("palette"),
                256 * 16,
                wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
            )
            .texture(
                Tag::new("depth"),
                wgpu::TextureFormat::Depth32Float,
                Scale::Full,
            )
            .assemble(device, width, height);

        Ok(Self {
            context,
            graph,
            geometry,
            outline,
            typography,
        })
    }

    pub fn measure(&mut self, text: &str, size: f32) -> f32 {
        self.typography.measure(text, size)
    }

    #[must_use]
    pub fn viewport(&self) -> Viewport {
        self.context.viewport()
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.context.resize(width, height);
        self.graph.resize(self.context.device(), width, height);
        self.typography.resize(self.context.queue(), width, height);
    }

    pub fn frame(&mut self, observer: Observer) -> Result<Frame<'_>> {
        self.geometry.clear();
        self.geometry.observe(observer);

        let viewport = self.context.viewport();
        self.outline.resize(viewport);

        Ok(Frame {
            renderer: self,
            labels: Vec::new(),
            outlined: false,
            background: None,
            observer,
            viewport,
        })
    }
}

pub struct Frame<'a> {
    renderer: &'a mut Renderer,
    labels: Vec<Label>,
    outlined: bool,
    background: Option<Color>,
    observer: Observer,
    viewport: Viewport,
}

impl Frame<'_> {
    #[must_use]
    pub fn clear(mut self, color: impl Into<Color>) -> Self {
        self.background = Some(color.into());
        self
    }

    #[must_use]
    pub fn label(mut self, label: Label) -> Self {
        self.labels.push(label);
        self
    }

    #[must_use]
    pub fn field(self, fields: impl IntoIterator<Item = field::Field>) -> Self {
        for f in fields {
            self.renderer.geometry.push(f);
        }
        self
    }

    #[must_use]
    pub fn outline(mut self) -> Self {
        self.outlined = true;
        self
    }

    pub fn render(self) -> Result<()> {
        if self.outlined {
            self.renderer.outline.enable();
        } else {
            self.renderer.outline.disable();
        }

        self.renderer.typography.feed(
            self.labels,
            self.observer.view,
            self.observer.projection,
            self.viewport,
        );

        let device = self.renderer.context.device();
        let queue = self.renderer.context.queue();
        let graph = &self.renderer.graph;

        let stages: &mut [&mut dyn Stage] = &mut [
            &mut self.renderer.geometry,
            &mut self.renderer.outline,
            &mut self.renderer.typography,
        ];

        for stage in stages.iter_mut() {
            stage.prepare(device, queue, graph)?;
        }

        let frame = self.renderer.context.frame()?;
        let mut encoder = self.renderer.context.encoder();

        if let Some(color) = self.background {
            frame.clear(&mut encoder, color);
        }

        let surface = frame.surface();

        for stage in stages.iter() {
            stage.draw(&mut encoder, surface, graph)?;
        }

        self.renderer.context.submit(encoder, frame);

        Ok(())
    }
}
