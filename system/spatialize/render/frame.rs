use color::Color;

pub struct Frame {
    output: wgpu::SurfaceTexture,
    surface: wgpu::TextureView,
}

impl Frame {
    #[must_use]
    pub fn new(output: wgpu::SurfaceTexture) -> Self {
        let surface = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        Self { output, surface }
    }

    #[must_use]
    pub fn surface(&self) -> &wgpu::TextureView {
        &self.surface
    }

    pub fn clear(&self, encoder: &mut wgpu::CommandEncoder, color: impl Into<Color>) {
        let background = color.into();
        let pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("render::clear"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &self.surface,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: f64::from(background.r),
                        g: f64::from(background.g),
                        b: f64::from(background.b),
                        a: f64::from(background.a),
                    }),
                    store: wgpu::StoreOp::Store,
                },
                depth_slice: None,
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
            multiview_mask: None,
        });
        drop(pass);
    }

    pub fn present(self) {
        self.output.present();
    }
}
