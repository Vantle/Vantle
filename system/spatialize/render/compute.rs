use binding::Binding;
use error::{Error, Result};

pub struct Compute {
    pipeline: wgpu::ComputePipeline,
    layout: wgpu::BindGroupLayout,
}

impl Compute {
    #[must_use]
    pub fn assembler() -> Assembler {
        Assembler::new()
    }

    #[must_use]
    pub fn binding(&self) -> &wgpu::BindGroupLayout {
        &self.layout
    }

    #[must_use]
    pub fn pipeline(&self) -> &wgpu::ComputePipeline {
        &self.pipeline
    }
}

pub struct Assembler {
    shader: Option<String>,
    bindings: Vec<(u32, Binding)>,
}

impl Assembler {
    #[must_use]
    pub fn new() -> Self {
        Self {
            shader: None,
            bindings: Vec::new(),
        }
    }

    #[must_use]
    pub fn shader(mut self, path: &str) -> Self {
        self.shader = Some(path.to_string());
        self
    }

    #[must_use]
    pub fn bind(mut self, index: u32, binding: Binding) -> Self {
        self.bindings.push((index, binding));
        self
    }

    pub fn assemble(self, device: &wgpu::Device) -> Result<Compute> {
        let path = self.shader.ok_or_else(|| Error::Configuration {
            field: "shader".into(),
        })?;

        let source = std::fs::read_to_string(&path).map_err(|source| Error::Shader { source })?;
        let module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("compute::shader"),
            source: wgpu::ShaderSource::Wgsl(source.into()),
        });

        let entries = self
            .bindings
            .iter()
            .map(|(index, binding)| binding.entry(*index))
            .collect::<Vec<_>>();

        let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("compute::layout"),
            entries: &entries,
        });

        let arrangement = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("compute::arrangement"),
            bind_group_layouts: &[&layout],
            immediate_size: 0,
        });

        let pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: Some("compute::pipeline"),
            layout: Some(&arrangement),
            module: &module,
            entry_point: Some("main"),
            compilation_options: wgpu::PipelineCompilationOptions::default(),
            cache: None,
        });

        Ok(Compute { pipeline, layout })
    }
}

impl Default for Assembler {
    fn default() -> Self {
        Self::new()
    }
}

pub struct Command<'a> {
    encoder: &'a mut wgpu::CommandEncoder,
    shader: &'a Compute,
    bindings: Vec<(u32, &'a wgpu::BindGroup)>,
}

impl<'a> Command<'a> {
    #[must_use]
    pub fn new(encoder: &'a mut wgpu::CommandEncoder, shader: &'a Compute) -> Self {
        Self {
            encoder,
            shader,
            bindings: Vec::new(),
        }
    }

    #[must_use]
    pub fn bind(mut self, slot: u32, group: &'a wgpu::BindGroup) -> Self {
        self.bindings.push((slot, group));
        self
    }

    pub fn dispatch(self, x: u32, y: u32, z: u32) -> Result<()> {
        {
            let mut pass = self
                .encoder
                .begin_compute_pass(&wgpu::ComputePassDescriptor {
                    label: Some("compute::dispatch"),
                    timestamp_writes: None,
                });

            pass.set_pipeline(&self.shader.pipeline);

            for (slot, group) in &self.bindings {
                pass.set_bind_group(*slot, *group, &[]);
            }

            pass.dispatch_workgroups(x, y, z);
        }

        Ok(())
    }
}
