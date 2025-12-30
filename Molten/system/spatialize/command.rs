use std::collections::HashMap;
use std::sync::Arc;

use wgpu::Instance;
use winit::event::{DeviceEvent, DeviceId, ElementState, Modifiers, WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowId};

use collision::Synchronization;
use dimension::Extent;
use effect::Effect;
use error::{Error, Result};
use field::Field;
use form::Geometry;
use graph::{edges, nodes};
use hypergraph::Label as Vertex;
use launcher::{Application, Launcher};
use layout::{Simulation, Tuning};
use mouse::State;
use observer::Observer;
use pane::Pane;
use primitive::Label;
use ray::Ray;
use render::Renderer;
use scene::Scene;
use view::View;

const BOUNDS: f32 = 500.0;

struct Spatialize {
    window: Arc<Window>,
    renderer: Renderer,
    scene: Scene,
    pane: Pane,
    mouse: State,
    view: View,
    collision: Effect<Geometry>,
    modifiers: Modifiers,
    widths: HashMap<Vertex, f32>,
}

impl Application for Spatialize {
    fn new(window: Arc<Window>, mut renderer: Renderer, aspect: f32) -> Result<Self> {
        let graph = mock::graph();
        let pane = Pane::default();

        let space = pane::extract(&graph, pane)?;
        let layout = Simulation::new(&space, BOUNDS, Tuning::default());

        let font = scale::font();
        let widths = space
            .nodes()
            .map(|node| (node.label(), renderer.measure(node.text(), font)))
            .collect::<HashMap<_, _>>();

        window.set_title(&pane.title());

        Ok(Self {
            window,
            renderer,
            scene: Scene::new(graph, space, layout),
            pane,
            mouse: State::new(),
            view: View::new(aspect),
            collision: Effect::new(),
            modifiers: Modifiers::default(),
            widths,
        })
    }

    #[expect(clippy::cast_precision_loss)]
    fn handle(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, window: WindowEvent) {
        let (width, height) = self.renderer.viewport().dimensions();
        let extent = Extent::new(width, height);

        match window {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }

            WindowEvent::Resized(size) => {
                self.renderer.resize(size.width, size.height);
                let aspect = size.width as f32 / size.height.max(1) as f32;
                self.view.resize(aspect);
                AsMut::<Simulation>::as_mut(&mut self.scene).reset();
            }

            WindowEvent::KeyboardInput { event, .. } => {
                if event.state == ElementState::Pressed
                    && event::keyboard(
                        &event.logical_key,
                        &mut self.pane,
                        &mut self.mouse,
                        &self.window,
                    )
                {
                    if let Err(e) = self.rebuild() {
                        tracing::error!("{:?}", miette::Report::new(e));
                    }
                    self.recompute();
                    self.window.request_redraw();
                }
            }

            WindowEvent::CursorMoved { position, .. } => {
                event::cursor(
                    position.x,
                    position.y,
                    &mut self.mouse,
                    &mut self.view,
                    extent,
                );
                self.probe();
                self.window.request_redraw();
            }

            WindowEvent::CursorLeft { .. } => {
                event::leave(&mut self.mouse);
                self.recompute();
                self.window.request_redraw();
            }

            WindowEvent::ModifiersChanged(modifiers) => {
                event::modifiers(
                    &self.modifiers,
                    &modifiers,
                    &self.mouse,
                    &mut self.view,
                    &self.window,
                    extent,
                );
                self.modifiers = modifiers;
            }

            WindowEvent::MouseInput { state, button, .. } => {
                if event::button(
                    state,
                    button,
                    &mut self.mouse,
                    &mut self.view,
                    &self.window,
                    extent,
                ) {
                    self.recompute();
                    self.window.request_redraw();
                }
            }

            WindowEvent::MouseWheel { delta, .. } => {
                event::scroll(delta, &mut self.view);
                self.window.request_redraw();
            }

            WindowEvent::PinchGesture { delta, .. } => {
                event::pinch(delta, &mut self.view);
                self.window.request_redraw();
            }

            WindowEvent::RotationGesture { delta, phase, .. } => {
                event::rotation(delta, phase, &mut self.view);
                self.window.request_redraw();
            }

            WindowEvent::RedrawRequested => {
                if !self.scene.converged() {
                    if let Err(e) = self.scene.step(BOUNDS) {
                        tracing::warn!("{:?}", miette::Report::new(e));
                    }
                    self.probe();
                }

                self.render();

                if !self.scene.converged() {
                    self.window.request_redraw();
                }
            }

            _ => {}
        }
    }

    #[expect(clippy::cast_possible_truncation)]
    fn motion(&mut self, _event: &ActiveEventLoop, _id: DeviceId, device: DeviceEvent) {
        if let DeviceEvent::MouseMotion { delta: (dx, dy) } = device
            && self.view.active()
        {
            self.view.pivot(dx as f32, dy as f32);
            self.view.drift(dx as f32, dy as f32);
            self.window.request_redraw();
        }
    }
}

impl Spatialize {
    fn rebuild(&mut self) -> Result<()> {
        let graph = self.scene.graph().clone();
        let space = pane::extract(&graph, self.pane)?;
        let layout = Simulation::new(&space, BOUNDS, Tuning::default());

        let font = scale::font();
        self.widths = space
            .nodes()
            .map(|node| (node.label(), self.renderer.measure(node.text(), font)))
            .collect::<HashMap<_, _>>();

        self.scene = Scene::new(graph, space, layout);
        Ok(())
    }

    fn recompute(&mut self) {
        let label = self.mouse.selected.or(self.mouse.hovered);
        let (graph, space) = self.scene.highlight();

        if let Some(hovered) = label {
            pane::highlight(space, graph, hovered, self.pane);
        } else {
            space.clear();
        }
    }

    fn probe(&mut self) {
        let Some((x, y)) = self.mouse.position else {
            return;
        };

        let (w, h) = self.renderer.viewport().dimensions();
        let Some(ray) = Ray::screen(x, y, w, h, &self.view.matrix(), &self.view.projection())
        else {
            return;
        };

        Synchronization::synchronize(
            &mut self.collision,
            self.scene.space(),
            self.scene.positions(),
            self.view.density(h),
            &self.widths,
        );

        let hit = self.collision.raycast(&ray);

        if hit != self.mouse.hovered {
            self.mouse.hovered = hit;
            self.recompute();
        }
    }

    fn render(&mut self) {
        let observer = Observer::new(
            self.view.matrix(),
            self.view.projection(),
            self.view.reference(),
        );

        let Ok(frame) = self.renderer.frame(observer) else {
            return;
        };

        let mut frame = frame.clear(palette::BACKGROUND);
        let mut fields = Vec::<Field>::new();

        let space = self.scene.space();
        let layout = self.scene.layout();
        let interacted = self.mouse.selected.or(self.mouse.hovered);

        for edge in edges(space, layout) {
            if Some(edge.label) == interacted {
                fields.push(edge.effect().into());
            } else {
                fields.push(edge.arrow.into());
            }
        }

        for vertex in nodes(space, layout) {
            let color = if vertex.highlighted {
                palette::HIGHLIGHTED
            } else {
                palette::TEXT
            };

            let text = vertex.text.clone();
            let offset = vertex.offset;

            if Some(vertex.label) == interacted {
                fields.push(vertex.effect().into());
            } else {
                fields.push(vertex.sphere.into());
            }

            let label = Label::new(&text, offset, scale::font(), color);
            frame = frame.label(label);
        }

        let _ = frame.field(fields).outline().render();
    }
}

fn main() -> Result<()> {
    let event = EventLoop::new().map_err(|source| Error::Event { source })?;
    event.set_control_flow(ControlFlow::Wait);

    let instance = Instance::new(&wgpu::InstanceDescriptor {
        backends: wgpu::Backends::all(),
        ..Default::default()
    });

    let mut launcher = Launcher::<Spatialize>::new(instance);

    event
        .run_app(&mut launcher)
        .map_err(|source| Error::Run { source })?;

    if let Some(error) = launcher.error() {
        return Err(error);
    }

    Ok(())
}
