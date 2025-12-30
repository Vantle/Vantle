use color::Color;
use vector::Vector;

pub const OUTLINE: u32 = 1 << 0;
pub const GLOW: u32 = 1 << 1;
pub const WIREFRAME: u32 = 1 << 2;

#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub color: [f32; 4],
    pub effects: u32,
    pub palette: u32,
    padding: [u32; 2],
}

impl Vertex {
    #[must_use]
    pub fn new(
        position: impl Into<Vector>,
        normal: impl Into<Vector>,
        color: impl Into<Color>,
        effects: u32,
        palette: u32,
    ) -> Self {
        Self {
            position: position.into().array(),
            normal: normal.into().array(),
            color: color.into().array(),
            effects,
            palette,
            padding: [0; 2],
        }
    }

    #[must_use]
    pub fn layout() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 6]>() as wgpu::BufferAddress,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 10]>() as wgpu::BufferAddress,
                    shader_location: 3,
                    format: wgpu::VertexFormat::Uint32,
                },
                wgpu::VertexAttribute {
                    offset: (std::mem::size_of::<[f32; 10]>() + std::mem::size_of::<u32>())
                        as wgpu::BufferAddress,
                    shader_location: 4,
                    format: wgpu::VertexFormat::Uint32,
                },
            ],
        }
    }
}
