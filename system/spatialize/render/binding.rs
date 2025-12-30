pub enum Binding {
    Uniform(wgpu::ShaderStages),
    Storage {
        stages: wgpu::ShaderStages,
        writable: bool,
    },
    Texture(wgpu::ShaderStages),
    Sampler(wgpu::ShaderStages),
}

impl Binding {
    #[must_use]
    pub fn uniform(stages: wgpu::ShaderStages) -> Self {
        Self::Uniform(stages)
    }

    #[must_use]
    pub fn storage(stages: wgpu::ShaderStages, writable: bool) -> Self {
        Self::Storage { stages, writable }
    }

    #[must_use]
    pub fn texture(stages: wgpu::ShaderStages) -> Self {
        Self::Texture(stages)
    }

    #[must_use]
    pub fn sampler(stages: wgpu::ShaderStages) -> Self {
        Self::Sampler(stages)
    }

    #[must_use]
    pub fn entry(&self, index: u32) -> wgpu::BindGroupLayoutEntry {
        match self {
            Self::Uniform(stages) => wgpu::BindGroupLayoutEntry {
                binding: index,
                visibility: *stages,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
            Self::Storage { stages, writable } => wgpu::BindGroupLayoutEntry {
                binding: index,
                visibility: *stages,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Storage {
                        read_only: !writable,
                    },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
            Self::Texture(stages) => wgpu::BindGroupLayoutEntry {
                binding: index,
                visibility: *stages,
                ty: wgpu::BindingType::Texture {
                    sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    view_dimension: wgpu::TextureViewDimension::D2,
                    multisampled: false,
                },
                count: None,
            },
            Self::Sampler(stages) => wgpu::BindGroupLayoutEntry {
                binding: index,
                visibility: *stages,
                ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                count: None,
            },
        }
    }
}
