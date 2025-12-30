use attachment::{Attachment, Depth};
use binding::Binding;
use error::{Error, Result};

pub struct Raster {
    pipeline: wgpu::RenderPipeline,
    layout: wgpu::BindGroupLayout,
}

impl Raster {
    #[must_use]
    pub fn assembler() -> Assembler {
        Assembler::new()
    }

    #[must_use]
    pub fn binding(&self) -> &wgpu::BindGroupLayout {
        &self.layout
    }

    #[must_use]
    pub fn pipeline(&self) -> &wgpu::RenderPipeline {
        &self.pipeline
    }
}

pub struct Assembler {
    shader: Option<String>,
    vertex: Option<wgpu::VertexBufferLayout<'static>>,
    bindings: Vec<(u32, Binding)>,
    targets: Vec<(wgpu::TextureFormat, Option<wgpu::BlendState>)>,
    depth: Option<(wgpu::TextureFormat, bool, wgpu::CompareFunction)>,
    cull: Option<wgpu::Face>,
}

impl Assembler {
    #[must_use]
    pub fn new() -> Self {
        Self {
            shader: None,
            vertex: None,
            bindings: Vec::new(),
            targets: Vec::new(),
            depth: None,
            cull: None,
        }
    }

    #[must_use]
    pub fn shader(mut self, path: &str) -> Self {
        self.shader = Some(path.to_string());
        self
    }

    #[must_use]
    pub fn vertex(mut self, layout: wgpu::VertexBufferLayout<'static>) -> Self {
        self.vertex = Some(layout);
        self
    }

    #[must_use]
    pub fn bind(mut self, index: u32, binding: Binding) -> Self {
        self.bindings.push((index, binding));
        self
    }

    #[must_use]
    pub fn target(mut self, format: wgpu::TextureFormat, blend: Option<wgpu::BlendState>) -> Self {
        self.targets.push((format, blend));
        self
    }

    #[must_use]
    pub fn depth(
        mut self,
        format: wgpu::TextureFormat,
        write: bool,
        compare: wgpu::CompareFunction,
    ) -> Self {
        self.depth = Some((format, write, compare));
        self
    }

    #[must_use]
    pub fn cull(mut self, face: wgpu::Face) -> Self {
        self.cull = Some(face);
        self
    }
}

impl Assembler {
    pub fn assemble(self, device: &wgpu::Device) -> Result<Raster> {
        let path = self.shader.ok_or_else(|| Error::Configuration {
            field: "shader".into(),
        })?;

        let source = std::fs::read_to_string(&path).map_err(|source| Error::Shader { source })?;
        let module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("raster::shader"),
            source: wgpu::ShaderSource::Wgsl(source.into()),
        });

        let entries = self
            .bindings
            .iter()
            .map(|(index, binding)| binding.entry(*index))
            .collect::<Vec<_>>();

        let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("raster::layout"),
            entries: &entries,
        });

        let arrangement = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("raster::arrangement"),
            bind_group_layouts: &[&layout],
            immediate_size: 0,
        });

        let targets = self
            .targets
            .iter()
            .map(|(format, blend)| {
                Some(wgpu::ColorTargetState {
                    format: *format,
                    blend: *blend,
                    write_mask: wgpu::ColorWrites::ALL,
                })
            })
            .collect::<Vec<_>>();

        let buffers = self
            .vertex
            .as_ref()
            .map_or(vec![], |layout| vec![layout.clone()]);

        let depth = self
            .depth
            .map(|(format, write, compare)| wgpu::DepthStencilState {
                format,
                depth_write_enabled: write,
                depth_compare: compare,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("raster::pipeline"),
            layout: Some(&arrangement),
            vertex: wgpu::VertexState {
                module: &module,
                entry_point: Some("vertex"),
                buffers: &buffers,
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &module,
                entry_point: Some("fragment"),
                targets: &targets,
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: self.cull,
                unclipped_depth: false,
                polygon_mode: wgpu::PolygonMode::Fill,
                conservative: false,
            },
            depth_stencil: depth,
            multisample: wgpu::MultisampleState::default(),
            multiview_mask: None,
            cache: None,
        });

        Ok(Raster { pipeline, layout })
    }
}

impl Default for Assembler {
    fn default() -> Self {
        Self::new()
    }
}

pub struct Command<'a> {
    encoder: &'a mut wgpu::CommandEncoder,
    shader: &'a Raster,
    bindings: Vec<(u32, &'a wgpu::BindGroup)>,
    vertices: Option<&'a wgpu::Buffer>,
    indices: Option<(&'a wgpu::Buffer, u32)>,
    targets: Vec<Attachment<'a>>,
    depth: Option<Depth<'a>>,
}

impl<'a> Command<'a> {
    #[must_use]
    pub fn new(encoder: &'a mut wgpu::CommandEncoder, shader: &'a Raster) -> Self {
        Self {
            encoder,
            shader,
            bindings: Vec::new(),
            vertices: None,
            indices: None,
            targets: Vec::new(),
            depth: None,
        }
    }

    #[must_use]
    pub fn bind(mut self, slot: u32, group: &'a wgpu::BindGroup) -> Self {
        self.bindings.push((slot, group));
        self
    }

    #[must_use]
    pub fn vertex(mut self, buffer: &'a wgpu::Buffer) -> Self {
        self.vertices = Some(buffer);
        self
    }

    #[must_use]
    pub fn index(mut self, buffer: &'a wgpu::Buffer, count: u32) -> Self {
        self.indices = Some((buffer, count));
        self
    }

    #[must_use]
    pub fn target(mut self, attachment: Attachment<'a>) -> Self {
        self.targets.push(attachment);
        self
    }

    #[must_use]
    pub fn depth(mut self, attachment: Depth<'a>) -> Self {
        self.depth = Some(attachment);
        self
    }

    pub fn draw(self) -> Result<()> {
        let attachments = self
            .targets
            .iter()
            .map(|attachment| Some(attachment.descriptor()))
            .collect::<Vec<_>>();

        let stencil = self.depth.as_ref().map(Depth::descriptor);

        {
            let mut pass = self.encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("raster::draw"),
                color_attachments: &attachments,
                depth_stencil_attachment: stencil,
                timestamp_writes: None,
                occlusion_query_set: None,
                multiview_mask: None,
            });

            pass.set_pipeline(&self.shader.pipeline);

            for (slot, group) in &self.bindings {
                pass.set_bind_group(*slot, *group, &[]);
            }

            if let Some(buffer) = self.vertices {
                pass.set_vertex_buffer(0, buffer.slice(..));
            }

            if let Some((buffer, count)) = self.indices {
                pass.set_index_buffer(buffer.slice(..), wgpu::IndexFormat::Uint32);
                pass.draw_indexed(0..count, 0, 0..1);
            } else {
                pass.draw(0..6, 0..1);
            }
        }

        Ok(())
    }
}
