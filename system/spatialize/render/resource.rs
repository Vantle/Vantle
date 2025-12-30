use tag::Tag;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Scale {
    Full,
    Half,
    Quarter,
}

impl Scale {
    #[must_use]
    pub fn apply(self, width: u32, height: u32) -> (u32, u32) {
        match self {
            Self::Full => (width.max(1), height.max(1)),
            Self::Half => ((width / 2).max(1), (height / 2).max(1)),
            Self::Quarter => ((width / 4).max(1), (height / 4).max(1)),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Descriptor {
    Texture {
        tag: Tag,
        format: wgpu::TextureFormat,
        scale: Scale,
        usage: wgpu::TextureUsages,
    },
    Buffer {
        tag: Tag,
        size: u64,
        usage: wgpu::BufferUsages,
    },
}

impl Descriptor {
    #[must_use]
    pub fn tag(&self) -> Tag {
        match self {
            Self::Texture { tag, .. } | Self::Buffer { tag, .. } => tag.clone(),
        }
    }
}

pub enum Resource {
    Texture {
        texture: wgpu::Texture,
        view: wgpu::TextureView,
        format: wgpu::TextureFormat,
        scale: Scale,
        usage: wgpu::TextureUsages,
    },
    Buffer {
        buffer: wgpu::Buffer,
    },
}

impl Resource {
    #[must_use]
    pub fn new(device: &wgpu::Device, descriptor: &Descriptor, width: u32, height: u32) -> Self {
        match descriptor {
            Descriptor::Texture {
                tag,
                format,
                scale,
                usage,
            } => {
                let label = tag.to_string();
                let (w, h) = scale.apply(width, height);
                let texture = device.create_texture(&wgpu::TextureDescriptor {
                    label: Some(&label),
                    size: wgpu::Extent3d {
                        width: w,
                        height: h,
                        depth_or_array_layers: 1,
                    },
                    mip_level_count: 1,
                    sample_count: 1,
                    dimension: wgpu::TextureDimension::D2,
                    format: *format,
                    usage: *usage,
                    view_formats: &[],
                });
                let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
                Self::Texture {
                    texture,
                    view,
                    format: *format,
                    scale: *scale,
                    usage: *usage,
                }
            }
            Descriptor::Buffer { tag, size, usage } => {
                let label = tag.to_string();
                let buffer = device.create_buffer(&wgpu::BufferDescriptor {
                    label: Some(&label),
                    size: *size,
                    usage: *usage,
                    mapped_at_creation: false,
                });
                Self::Buffer { buffer }
            }
        }
    }

    #[must_use]
    pub fn view(&self) -> Option<&wgpu::TextureView> {
        match self {
            Self::Texture { view, .. } => Some(view),
            Self::Buffer { .. } => None,
        }
    }

    #[must_use]
    pub fn handle(&self) -> Option<&wgpu::Buffer> {
        match self {
            Self::Buffer { buffer, .. } => Some(buffer),
            Self::Texture { .. } => None,
        }
    }

    pub fn resize(&mut self, device: &wgpu::Device, width: u32, height: u32) {
        if let Self::Texture {
            texture,
            view,
            format,
            scale,
            usage,
        } = self
        {
            let (w, h) = scale.apply(width, height);
            *texture = device.create_texture(&wgpu::TextureDescriptor {
                label: Some("resource::texture"),
                size: wgpu::Extent3d {
                    width: w,
                    height: h,
                    depth_or_array_layers: 1,
                },
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: *format,
                usage: *usage,
                view_formats: &[],
            });
            *view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        }
    }
}
