use clap::Parser;
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowId};

#[derive(Parser, Debug)]
#[command(name = "spatialize")]
#[command(about = "Molten visualization")]
pub struct Arguments {
    #[arg(long)]
    pub source: Option<String>,

    #[arg(long)]
    pub record: Option<String>,
}

struct Application {
    window: Option<Window>,
}

impl Application {
    fn new() -> Self {
        Self { window: None }
    }
}

impl ApplicationHandler for Application {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let attributes = Window::default_attributes().with_title("Spatialize");
        self.window = Some(event_loop.create_window(attributes).unwrap());
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                if let Some(window) = &self.window {
                    window.request_redraw();
                }
            }
            _ => {}
        }
    }
}

pub fn run(_arguments: Arguments) {
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Wait);

    let mut app = Application::new();
    event_loop.run_app(&mut app).unwrap();
}

fn main() {
    let arguments = Arguments::parse();
    run(arguments);
}
