use std::sync::Arc;

use wgpu::Instance;
use winit::application::ApplicationHandler;
use winit::dpi::LogicalSize;
use winit::event::{DeviceEvent, DeviceId, WindowEvent};
use winit::event_loop::ActiveEventLoop;
use winit::window::{Icon, Window, WindowId};

use error::{Error, Result};
use pane::Pane;
use proportion::scale;
use render::Renderer;

const WINDOW: f32 = 1000.0;
const LOGO: &str = "Molten/resource/logo.png";

fn icon() -> Option<Icon> {
    let image = image::open(LOGO).ok()?.into_rgba8();
    let (width, height) = image.dimensions();
    Icon::from_rgba(image.into_raw(), width, height).ok()
}

pub struct Launcher<A>
where
    A: Application,
{
    instance: Instance,
    application: Option<A>,
    error: Option<Error>,
}

impl<A> Launcher<A>
where
    A: Application,
{
    #[must_use]
    pub fn new(instance: Instance) -> Self {
        Self {
            instance,
            application: None,
            error: None,
        }
    }

    #[must_use]
    pub fn error(self) -> Option<Error> {
        self.error
    }

    #[expect(clippy::cast_precision_loss)]
    fn initialize(&mut self, event: &ActiveEventLoop) -> Result<A> {
        let mut attributes = Window::default_attributes()
            .with_title(Pane::default().title())
            .with_inner_size(LogicalSize::new(WINDOW, WINDOW / scale().compute()));

        if let Some(icon) = icon() {
            attributes = attributes.with_window_icon(Some(icon));
        }

        let window = Arc::new(
            event
                .create_window(attributes)
                .map_err(|source| Error::Window { source })?,
        );

        let size = window.inner_size();

        let surface = self
            .instance
            .create_surface(window.clone())
            .map_err(|source| Error::Surface { source })?;

        let adapter =
            pollster::block_on(self.instance.request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            }))
            .map_err(|source| Error::Adapter { source })?;

        let renderer = pollster::block_on(
            render::Assembler::new()
                .surface(surface)
                .adapter(adapter)
                .size(size.width, size.height)
                .assemble(),
        )?;

        let aspect = size.width as f32 / size.height.max(1) as f32;

        A::new(window, renderer, aspect)
    }
}

impl<A> ApplicationHandler for Launcher<A>
where
    A: Application,
{
    fn resumed(&mut self, event: &ActiveEventLoop) {
        if self.application.is_some() || self.error.is_some() {
            return;
        }

        match self.initialize(event) {
            Ok(application) => self.application = Some(application),
            Err(error) => {
                self.error = Some(error);
                event.exit();
            }
        }
    }

    fn window_event(&mut self, event: &ActiveEventLoop, id: WindowId, window: WindowEvent) {
        if let Some(application) = &mut self.application {
            application.handle(event, id, window);
        }
    }

    fn device_event(&mut self, event: &ActiveEventLoop, id: DeviceId, device: DeviceEvent) {
        if let Some(application) = &mut self.application {
            application.motion(event, id, device);
        }
    }
}

pub trait Application: Sized {
    fn new(window: Arc<Window>, renderer: Renderer, aspect: f32) -> Result<Self>;
    fn handle(&mut self, event: &ActiveEventLoop, id: WindowId, window: WindowEvent);
    fn motion(&mut self, event: &ActiveEventLoop, id: DeviceId, device: DeviceEvent);
}
