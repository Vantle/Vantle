use std::sync::Arc;

use dimension::Extent;
use winit::event::{ElementState, Modifiers, MouseButton, MouseScrollDelta, TouchPhase};
use winit::keyboard::{Key, NamedKey};
use winit::window::{CursorGrabMode, Window};

use mouse::State;
use pane::Pane;
use view::{Mode, View};

pub fn grab(window: &Window) {
    window.focus_window();
    let locked = window.set_cursor_grab(CursorGrabMode::Locked).is_ok()
        || window.set_cursor_grab(CursorGrabMode::Confined).is_ok();
    if locked {
        window.set_cursor_visible(false);
    }
}

pub fn release(window: &Window) {
    let _ = window.set_cursor_grab(CursorGrabMode::None);
    window.set_cursor_visible(true);
}

pub fn keyboard(key: &Key, pane: &mut Pane, mouse: &mut State, window: &Arc<Window>) -> bool {
    match key {
        Key::Named(NamedKey::Tab) => {
            *pane = pane.toggle();
            window.set_title(&pane.title());
            true
        }
        Key::Named(NamedKey::Escape) => {
            mouse.deselect();
            true
        }
        Key::Character(c) if c.as_str() == "r" => {
            *pane = Pane::Relation;
            window.set_title(&pane.title());
            true
        }
        Key::Character(c) if c.as_str() == "i" => {
            *pane = Pane::Inference;
            window.set_title(&pane.title());
            true
        }
        _ => false,
    }
}

#[expect(clippy::cast_possible_truncation)]
pub fn cursor(x: f64, y: f64, mouse: &mut State, view: &mut View, extent: Extent) {
    mouse.moved(x, y);

    if view.active() {
        let (nx, ny) = View::normalize(x as f32, y as f32, extent.width, extent.height);
        view.rotate(nx, ny);
        view.pan(nx, ny);
    }
}

pub fn leave(mouse: &mut State) {
    mouse.exited();
}

pub fn modifiers(
    previous: &Modifiers,
    current: &Modifiers,
    mouse: &State,
    view: &mut View,
    window: &Window,
    extent: Extent,
) {
    let was = previous.state().control_key();
    let now = current.state().control_key();

    if now && !was {
        if let Some((x, y)) = mouse.position {
            grab(window);
            let (nx, ny) = View::normalize(x, y, extent.width, extent.height);
            view.begin(Mode::Rotate, nx, ny);
        }
    } else if !now && was {
        release(window);
        view.end(Mode::Rotate);
    }
}

pub fn button(
    state: ElementState,
    button: MouseButton,
    mouse: &mut State,
    view: &mut View,
    window: &Window,
    extent: Extent,
) -> bool {
    match (state, button) {
        (ElementState::Pressed, MouseButton::Left) => {
            if let Some((x, y)) = mouse.position {
                grab(window);
                let (nx, ny) = View::normalize(x, y, extent.width, extent.height);
                view.begin(Mode::Pan, nx, ny);
            }
            false
        }
        (ElementState::Released, MouseButton::Left) => {
            release(window);
            view.end(Mode::Pan);
            false
        }
        (ElementState::Pressed, MouseButton::Middle) => {
            if let Some((x, y)) = mouse.position {
                grab(window);
                let (nx, ny) = View::normalize(x, y, extent.width, extent.height);
                view.begin(Mode::Rotate, nx, ny);
            }
            false
        }
        (ElementState::Released, MouseButton::Middle) => {
            release(window);
            view.end(Mode::Rotate);
            false
        }
        (ElementState::Pressed, MouseButton::Right) => {
            mouse.select();
            true
        }
        _ => false,
    }
}

#[expect(clippy::cast_possible_truncation)]
pub fn scroll(delta: MouseScrollDelta, view: &mut View) {
    let (dx, dy) = match delta {
        MouseScrollDelta::LineDelta(x, y) => (x * 0.05, y * 0.05),
        MouseScrollDelta::PixelDelta(p) => (p.x as f32 / 500.0, p.y as f32 / 500.0),
    };
    view.scroll(-dx, dy);
}

#[expect(clippy::cast_possible_truncation)]
pub fn pinch(delta: f64, view: &mut View) {
    view.zoom(delta as f32 * 2.0);
}

pub fn rotation(delta: f32, phase: TouchPhase, view: &mut View) {
    view.twist(delta, phase == TouchPhase::Ended);
}
