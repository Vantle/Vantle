use effect::Effect;
use field::Field;
use palette::Palette;
use primitive::{Cone, Cylinder, Geometry, Sphere};
use vertex::{OUTLINE, Vertex};

#[derive(Debug, Clone, Copy)]
pub struct Resolution {
    pub latitude: u32,
    pub longitude: u32,
    pub segments: u32,
}

impl Default for Resolution {
    fn default() -> Self {
        Self {
            latitude: 12,
            longitude: 24,
            segments: 16,
        }
    }
}

struct Entry<T> {
    primitive: T,
    effects: u32,
    index: u32,
}

pub struct Batch {
    spheres: Vec<Entry<Sphere>>,
    cylinders: Vec<Entry<Cylinder>>,
    cones: Vec<Entry<Cone>>,
    resolution: Resolution,
}

pub struct Buffers {
    pub vertex: wgpu::Buffer,
    pub index: wgpu::Buffer,
    pub count: u32,
}

impl Batch {
    #[must_use]
    pub fn new(resolution: Resolution) -> Self {
        Self {
            spheres: Vec::new(),
            cylinders: Vec::new(),
            cones: Vec::new(),
            resolution,
        }
    }

    pub fn push(&mut self, shape: Geometry) {
        match shape {
            Geometry::Sphere(sphere) => self.spheres.push(Entry {
                primitive: sphere,
                effects: 0,
                index: 0,
            }),
            Geometry::Cylinder(cylinder) => self.cylinders.push(Entry {
                primitive: cylinder,
                effects: 0,
                index: 0,
            }),
            Geometry::Cone(cone) => self.cones.push(Entry {
                primitive: cone,
                effects: 0,
                index: 0,
            }),
            Geometry::Label(_) => {}
        }
    }

    pub fn field(&mut self, field: &Field, palette: &mut Palette, effects: u32, index: u32) {
        match field {
            Field::Sphere(sphere) => self.spheres.push(Entry {
                primitive: *sphere,
                effects,
                index,
            }),
            Field::Cylinder(cylinder) => self.cylinders.push(Entry {
                primitive: *cylinder,
                effects,
                index,
            }),
            Field::Cone(cone) => self.cones.push(Entry {
                primitive: *cone,
                effects,
                index,
            }),
            Field::Union { left, right }
            | Field::Intersect { left, right }
            | Field::Subtract { left, right } => {
                self.field(left, palette, effects, index);
                self.field(right, palette, effects, index);
            }
            Field::Effected {
                base,
                effects: applied,
            } => {
                let (new_effects, new_index) = resolve(applied, palette);
                self.field(base, palette, effects | new_effects, new_index);
            }
        }
    }

    pub fn clear(&mut self) {
        self.spheres.clear();
        self.cylinders.clear();
        self.cones.clear();
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.spheres.is_empty() && self.cylinders.is_empty() && self.cones.is_empty()
    }

    #[must_use]
    pub fn tessellate(&self) -> (Vec<Vertex>, Vec<u32>) {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        for entry in &self.cylinders {
            tessellate::cylinder(
                &mut vertices,
                &mut indices,
                &entry.primitive,
                entry.effects,
                entry.index,
                self.resolution.segments,
            );
        }

        for entry in &self.cones {
            tessellate::cone(
                &mut vertices,
                &mut indices,
                &entry.primitive,
                entry.effects,
                entry.index,
                self.resolution.segments,
            );
        }

        for entry in &self.spheres {
            tessellate::sphere(
                &mut vertices,
                &mut indices,
                &entry.primitive,
                entry.effects,
                entry.index,
                self.resolution.latitude,
                self.resolution.longitude,
            );
        }

        (vertices, indices)
    }

    #[must_use]
    pub fn buffers(&self, device: &wgpu::Device) -> Buffers {
        use wgpu::util::DeviceExt;

        let (vertices, indices) = self.tessellate();

        if vertices.is_empty() || indices.is_empty() {
            return Buffers {
                vertex: device.create_buffer(&wgpu::BufferDescriptor {
                    label: Some("render::batch::vertex"),
                    size: 4,
                    usage: wgpu::BufferUsages::VERTEX,
                    mapped_at_creation: false,
                }),
                index: device.create_buffer(&wgpu::BufferDescriptor {
                    label: Some("render::batch::index"),
                    size: 4,
                    usage: wgpu::BufferUsages::INDEX,
                    mapped_at_creation: false,
                }),
                count: 0,
            };
        }

        let vertex = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("render::batch::vertex"),
            contents: bytemuck::cast_slice(&vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let index = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("render::batch::index"),
            contents: bytemuck::cast_slice(&indices),
            usage: wgpu::BufferUsages::INDEX,
        });

        Buffers {
            vertex,
            index,
            count: u32::try_from(indices.len()).unwrap_or(u32::MAX),
        }
    }
}

fn resolve(effects: &[Effect], palette: &mut Palette) -> (u32, u32) {
    let mut flags = 0u32;
    let mut index = 0u32;

    for effect in effects {
        match effect {
            Effect::Outline(outline) => {
                flags |= OUTLINE;
                index = palette.insert(outline.color, outline.width);
            }
        }
    }

    (flags, index)
}

impl Default for Batch {
    fn default() -> Self {
        Self::new(Resolution::default())
    }
}
