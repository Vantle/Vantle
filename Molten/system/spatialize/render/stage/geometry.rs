use std::hash::{Hash, Hasher};

use attachment::{Attachment, Depth};
use batch::{Batch, Buffers, Resolution};
use binding::Binding;
use error::{Error, Result};
use field::Field;
use graph::Graph;
use observer::Observer;
use palette::Palette;
use raster::Raster;
use stage::Stage;
use tag::Tag;
use uniform::Uniform;
use vertex::Vertex;

const SHADER: &str = "Molten/resource/system/spatialize/pipeline.wgsl";

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

    pub fn assemble(self) -> Result<Geometry> {
        let device = self.device.ok_or_else(|| Error::Configuration {
            field: "device".into(),
        })?;
        let format = self.format.ok_or_else(|| Error::Configuration {
            field: "format".into(),
        })?;

        let shader = Raster::assembler()
            .shader(SHADER)
            .vertex(Vertex::layout())
            .bind(
                0,
                Binding::uniform(wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT),
            )
            .target(format, Some(wgpu::BlendState::ALPHA_BLENDING))
            .target(wgpu::TextureFormat::Rg8Unorm, None)
            .depth(
                wgpu::TextureFormat::Depth32Float,
                true,
                wgpu::CompareFunction::Less,
            )
            .cull(wgpu::Face::Back)
            .assemble(device)?;

        let uniform = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("geometry::uniform"),
            size: std::mem::size_of::<Uniform>() as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("geometry::group"),
            layout: shader.binding(),
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform.as_entire_binding(),
            }],
        });

        Ok(Geometry {
            shader,
            group,
            uniform,
            fields: Vec::new(),
            observer: None,
            buffers: None,
            resolution: Resolution::default(),
            fingerprint: 0,
            colors: [[0.0; 4]; 256],
        })
    }
}

impl Default for Assembler<'_> {
    fn default() -> Self {
        Self::new()
    }
}

pub struct Geometry {
    shader: Raster,
    group: wgpu::BindGroup,
    uniform: wgpu::Buffer,
    fields: Vec<Field>,
    observer: Option<Observer>,
    buffers: Option<Buffers>,
    resolution: Resolution,
    fingerprint: u64,
    colors: [[f32; 4]; 256],
}

impl Geometry {
    pub fn push(&mut self, field: impl Into<Field>) {
        self.fields.push(field.into());
    }

    pub fn observe(&mut self, observer: Observer) {
        self.observer = Some(observer);
    }

    pub fn clear(&mut self) {
        self.fields.clear();
    }

    pub fn reset(&mut self) {
        self.fields.clear();
        self.fingerprint = 0;
        self.colors = [[0.0; 4]; 256];
        self.buffers = None;
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.fields.is_empty()
    }

    pub fn prepare(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        graph: &Graph,
    ) -> Result<()> {
        if let Some(observer) = self.observer {
            let data = Uniform::matrices(observer.view, observer.projection, observer.reference);
            queue.write_buffer(&self.uniform, 0, bytemuck::bytes_of(&data));
        }

        if self.fields.is_empty() {
            self.buffers = None;
            return Ok(());
        }

        let current = fingerprint(&self.fields);

        if current != self.fingerprint || self.buffers.is_none() {
            let mut batch = Batch::new(self.resolution);
            let mut palette = Palette::new();

            for field in &self.fields {
                batch.field(field, &mut palette, 0, 0);
            }

            self.colors = palette.buffer();
            self.buffers = Some(batch.buffers(device));
            self.fingerprint = current;
        }

        let buffer = graph.buffer(Tag::new("palette"))?;
        queue.write_buffer(buffer, 0, bytemuck::cast_slice(&self.colors));

        Ok(())
    }

    pub fn draw(
        &self,
        encoder: &mut wgpu::CommandEncoder,
        surface: &wgpu::TextureView,
        graph: &Graph,
    ) -> Result<()> {
        let Some(buffers) = &self.buffers else {
            return Ok(());
        };

        if buffers.count == 0 {
            return Ok(());
        }

        let mask = graph.texture(Tag::new("mask"))?;
        let depth = graph.texture(Tag::new("depth"))?;

        raster::Command::new(encoder, &self.shader)
            .bind(0, &self.group)
            .vertex(&buffers.vertex)
            .index(&buffers.index, buffers.count)
            .target(Attachment::Load(surface))
            .target(Attachment::Clear(mask, wgpu::Color::BLACK))
            .depth(Depth::Clear(depth))
            .draw()
    }
}

fn fingerprint(fields: &[Field]) -> u64 {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    fields.hash(&mut hasher);
    hasher.finish()
}

impl Stage for Geometry {
    fn prepare(&mut self, device: &wgpu::Device, queue: &wgpu::Queue, graph: &Graph) -> Result<()> {
        self.prepare(device, queue, graph)
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
