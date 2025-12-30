pub enum Attachment<'a> {
    Load(&'a wgpu::TextureView),
    Clear(&'a wgpu::TextureView, wgpu::Color),
}

impl Attachment<'_> {
    #[must_use]
    pub fn descriptor(&self) -> wgpu::RenderPassColorAttachment<'_> {
        match self {
            Self::Load(view) => wgpu::RenderPassColorAttachment {
                view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Load,
                    store: wgpu::StoreOp::Store,
                },
                depth_slice: None,
            },
            Self::Clear(view, color) => wgpu::RenderPassColorAttachment {
                view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(*color),
                    store: wgpu::StoreOp::Store,
                },
                depth_slice: None,
            },
        }
    }
}

pub enum Depth<'a> {
    Load(&'a wgpu::TextureView),
    Clear(&'a wgpu::TextureView),
}

impl Depth<'_> {
    #[must_use]
    pub fn descriptor(&self) -> wgpu::RenderPassDepthStencilAttachment<'_> {
        match self {
            Self::Load(view) => wgpu::RenderPassDepthStencilAttachment {
                view,
                depth_ops: Some(wgpu::Operations {
                    load: wgpu::LoadOp::Load,
                    store: wgpu::StoreOp::Store,
                }),
                stencil_ops: None,
            },
            Self::Clear(view) => wgpu::RenderPassDepthStencilAttachment {
                view,
                depth_ops: Some(wgpu::Operations {
                    load: wgpu::LoadOp::Clear(1.0),
                    store: wgpu::StoreOp::Store,
                }),
                stencil_ops: None,
            },
        }
    }
}
