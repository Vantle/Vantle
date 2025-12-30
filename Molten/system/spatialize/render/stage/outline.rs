use attachment::Attachment;
use binding::Binding;
use dimension::Viewport;
use error::{Error, Result};
use graph::Graph;
use raster::Raster;
use stage::Stage;
use tag::Tag;

const SHADER: &str = "Molten/resource/system/spatialize/outline.wgsl";

pub struct Assembler<'a> {
    device: Option<&'a wgpu::Device>,
    format: Option<wgpu::TextureFormat>,
}

impl<'a> Assembler<'a> {
    #[must_use]
    pub fn new() -> Self {
        Self {
            device: None,
            format: None,
        }
    }

    #[must_use]
    pub fn device(mut self, device: &'a wgpu::Device) -> Self {
        self.device = Some(device);
        self
    }

    #[must_use]
    pub fn format(mut self, format: wgpu::TextureFormat) -> Self {
        self.format = Some(format);
        self
    }

    pub fn assemble(self) -> Result<Outline> {
        let device = self.device.ok_or_else(|| Error::Configuration {
            field: "device".into(),
        })?;
        let format = self.format.ok_or_else(|| Error::Configuration {
            field: "format".into(),
        })?;

        let shader = Raster::assembler()
            .shader(SHADER)
            .bind(0, Binding::texture(wgpu::ShaderStages::FRAGMENT))
            .bind(1, Binding::sampler(wgpu::ShaderStages::FRAGMENT))
            .bind(2, Binding::uniform(wgpu::ShaderStages::FRAGMENT))
            .bind(3, Binding::storage(wgpu::ShaderStages::FRAGMENT, false))
            .target(format, Some(wgpu::BlendState::ALPHA_BLENDING))
            .assemble(device)?;

        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("outline::sampler"),
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::MipmapFilterMode::Nearest,
            ..Default::default()
        });

        let config = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("outline::config"),
            size: 16,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        Ok(Outline {
            shader,
            sampler,
            config,
            group: None,
            enabled: false,
            viewport: Viewport::new(1, 1),
        })
    }
}

impl Default for Assembler<'_> {
    fn default() -> Self {
        Self::new()
    }
}

pub struct Outline {
    shader: Raster,
    sampler: wgpu::Sampler,
    config: wgpu::Buffer,
    group: Option<wgpu::BindGroup>,
    enabled: bool,
    viewport: Viewport,
}

impl Outline {
    pub fn enable(&mut self) {
        self.enabled = true;
    }

    pub fn disable(&mut self) {
        self.enabled = false;
    }

    pub fn resize(&mut self, viewport: Viewport) {
        self.viewport = viewport;
    }

    pub fn prepare(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        graph: &Graph,
    ) -> Result<()> {
        if !self.enabled {
            self.group = None;
            return Ok(());
        }

        let mask = graph.texture(Tag::new("mask"))?;
        let palette = graph.buffer(Tag::new("palette"))?;

        let (width, height) = self.viewport.dimensions();
        let data: [f32; 4] = [width, height, 1.0, 0.0];
        queue.write_buffer(&self.config, 0, bytemuck::cast_slice(&data));

        self.group = Some(device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("outline::group"),
            layout: self.shader.binding(),
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(mask),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&self.sampler),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: self.config.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 3,
                    resource: palette.as_entire_binding(),
                },
            ],
        }));
        Ok(())
    }

    pub fn draw(
        &self,
        encoder: &mut wgpu::CommandEncoder,
        surface: &wgpu::TextureView,
    ) -> Result<()> {
        let Some(group) = &self.group else {
            return Ok(());
        };

        raster::Command::new(encoder, &self.shader)
            .bind(0, group)
            .target(Attachment::Load(surface))
            .draw()
    }
}

impl Stage for Outline {
    fn prepare(&mut self, device: &wgpu::Device, queue: &wgpu::Queue, graph: &Graph) -> Result<()> {
        self.prepare(device, queue, graph)
    }

    fn draw(
        &self,
        encoder: &mut wgpu::CommandEncoder,
        surface: &wgpu::TextureView,
        _graph: &Graph,
    ) -> Result<()> {
        self.draw(encoder, surface)
    }
}
