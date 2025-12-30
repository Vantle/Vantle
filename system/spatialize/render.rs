pub use error;

use assemble::Async;
use dimension::Viewport;
use error::{Error, Result};
use frame::Frame;

pub struct Context {
    device: wgpu::Device,
    queue: wgpu::Queue,
    surface: wgpu::Surface<'static>,
    config: wgpu::SurfaceConfiguration,
}

pub struct Assembler {
    surface: Option<wgpu::Surface<'static>>,
    adapter: Option<wgpu::Adapter>,
    size: Option<(u32, u32)>,
}

impl Assembler {
    #[must_use]
    pub fn new() -> Self {
        Self {
            surface: None,
            adapter: None,
            size: None,
        }
    }

    #[must_use]
    pub fn surface(mut self, surface: wgpu::Surface<'static>) -> Self {
        self.surface = Some(surface);
        self
    }

    #[must_use]
    pub fn adapter(mut self, adapter: wgpu::Adapter) -> Self {
        self.adapter = Some(adapter);
        self
    }

    #[must_use]
    pub fn size(mut self, width: u32, height: u32) -> Self {
        self.size = Some((width, height));
        self
    }

    pub async fn assemble(self) -> Result<Context> {
        let surface = self.surface.ok_or(Error::Configuration {
            field: "surface".into(),
        })?;
        let adapter = self.adapter.ok_or(Error::Configuration {
            field: "adapter".into(),
        })?;
        let (width, height) = self.size.ok_or(Error::Configuration {
            field: "size".into(),
        })?;

        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor::default())
            .await
            .map_err(|source| Error::Device { source })?;

        let capabilities = surface.get_capabilities(&adapter);
        let format = capabilities
            .formats
            .iter()
            .find(|f| f.is_srgb())
            .copied()
            .unwrap_or(capabilities.formats[0]);

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format,
            width: width.max(1),
            height: height.max(1),
            present_mode: wgpu::PresentMode::AutoVsync,
            alpha_mode: capabilities.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        surface.configure(&device, &config);

        Ok(Context {
            device,
            queue,
            surface,
            config,
        })
    }
}

impl Async for Assembler {
    type Output = Result<Context>;

    fn assemble(self) -> impl std::future::Future<Output = Self::Output> + Send {
        Self::assemble(self)
    }
}

impl Default for Assembler {
    fn default() -> Self {
        Self::new()
    }
}

impl Context {
    pub fn device(&self) -> &wgpu::Device {
        &self.device
    }

    pub fn queue(&self) -> &wgpu::Queue {
        &self.queue
    }

    pub fn format(&self) -> wgpu::TextureFormat {
        self.config.format
    }

    #[must_use]
    pub fn viewport(&self) -> Viewport {
        Viewport::new(self.config.width, self.config.height)
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        if width > 0 && height > 0 {
            self.config.width = width;
            self.config.height = height;
            self.surface.configure(&self.device, &self.config);
        }
    }

    pub fn frame(&self) -> Result<Frame> {
        let output = self
            .surface
            .get_current_texture()
            .map_err(|source| Error::Swapchain { source })?;

        Ok(Frame::new(output))
    }

    #[must_use]
    pub fn encoder(&self) -> wgpu::CommandEncoder {
        self.device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("render::encoder"),
            })
    }

    pub fn submit(&self, encoder: wgpu::CommandEncoder, frame: Frame) {
        self.queue.submit(std::iter::once(encoder.finish()));
        frame.present();
    }
}
