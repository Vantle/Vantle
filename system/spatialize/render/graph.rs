use std::collections::HashMap;

use error::{Error, Result};
use resource::{Descriptor, Resource, Scale};
use tag::Tag;

pub struct Graph {
    resources: HashMap<Tag, Resource>,
}

impl Graph {
    #[must_use]
    pub fn assembler() -> Assembler {
        Assembler::new()
    }

    pub fn texture(&self, tag: Tag) -> Result<&wgpu::TextureView> {
        let label = tag.to_string();
        self.resources
            .get(&tag)
            .ok_or_else(|| Error::Resource { tag: label.clone() })?
            .view()
            .ok_or(Error::Mismatch {
                tag: label,
                expected: "texture".into(),
                found: "buffer".into(),
            })
    }

    pub fn buffer(&self, tag: Tag) -> Result<&wgpu::Buffer> {
        let label = tag.to_string();
        self.resources
            .get(&tag)
            .ok_or_else(|| Error::Resource { tag: label.clone() })?
            .handle()
            .ok_or(Error::Mismatch {
                tag: label,
                expected: "buffer".into(),
                found: "texture".into(),
            })
    }

    pub fn resize(&mut self, device: &wgpu::Device, width: u32, height: u32) {
        for resource in self.resources.values_mut() {
            resource.resize(device, width, height);
        }
    }
}

pub struct Assembler {
    descriptors: Vec<Descriptor>,
}

impl Assembler {
    #[must_use]
    pub fn new() -> Self {
        Self {
            descriptors: Vec::new(),
        }
    }

    #[must_use]
    pub fn texture(mut self, tag: Tag, format: wgpu::TextureFormat, scale: Scale) -> Self {
        self.descriptors.push(Descriptor::Texture {
            tag,
            format,
            scale,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
        });
        self
    }

    #[must_use]
    pub fn buffer(mut self, tag: Tag, size: u64, usage: wgpu::BufferUsages) -> Self {
        self.descriptors
            .push(Descriptor::Buffer { tag, size, usage });
        self
    }

    #[must_use]
    pub fn assemble(self, device: &wgpu::Device, width: u32, height: u32) -> Graph {
        let mut resources = HashMap::new();
        for descriptor in &self.descriptors {
            let resource = Resource::new(device, descriptor, width, height);
            resources.insert(descriptor.tag(), resource);
        }
        Graph { resources }
    }
}

impl Default for Assembler {
    fn default() -> Self {
        Self::new()
    }
}
