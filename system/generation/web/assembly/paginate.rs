use std::{cell::RefCell, rc::Rc};

use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::Document;

const THRESHOLD: f64 = 300.0;

thread_local! {
    static INSTANCE: RefCell<Option<Instance>> = const { RefCell::new(None) };
}

struct Instance {
    #[expect(dead_code)]
    state: Rc<RefCell<Scroll>>,
    handler: Option<Closure<dyn FnMut(web_sys::Event)>>,
    #[expect(dead_code)]
    observer: Option<Closure<dyn FnMut(web_sys::Event)>>,
}

struct Scroll {
    document: Document,
    capacity: u32,
    total: u32,
    revealed: u32,
}

fn teardown() {
    INSTANCE.with(|cell| {
        let Some(previous) = cell.borrow_mut().take() else {
            return;
        };
        if let Some(handler) = &previous.handler
            && let Some(window) = web_sys::window()
        {
            let function: &js_sys::Function = handler.as_ref().unchecked_ref();
            let _ = window.remove_event_listener_with_callback("scroll", function);
        }
    });
}

pub fn initialize(document: &Document) {
    let grid = dashboard::grid().selector();
    let Ok(Some(container)) = document.query_selector(&grid) else {
        teardown();
        return;
    };

    if container.get_attribute(attribute::bound().name()).is_some() {
        teardown();
        return;
    }

    teardown();

    let Some(capacity) = container
        .get_attribute(attribute::capacity().name())
        .and_then(|value| value.parse::<u32>().ok())
    else {
        return;
    };

    let ordinals = attribute::ordinal().selector();
    let Ok(elements) = document.query_selector_all(&ordinals) else {
        return;
    };

    let total = elements.length();
    if total == 0 {
        return;
    }

    let _ = container.set_attribute(attribute::bound().name(), "");

    let revealed = capacity.min(total);
    let scroll = Rc::new(RefCell::new(Scroll {
        document: document.clone(),
        capacity,
        total,
        revealed,
    }));

    apply(&scroll.borrow(), 0);

    let handler = if total > revealed {
        let Some(window) = web_sys::window() else {
            return;
        };

        let weak = Rc::downgrade(&scroll);
        let closure = Closure::<dyn FnMut(web_sys::Event)>::new(move |_: web_sys::Event| {
            let Some(scroll) = weak.upgrade() else {
                return;
            };
            advance(&scroll);
        });
        let function: &js_sys::Function = closure.as_ref().unchecked_ref();
        let _ = window.add_event_listener_with_callback("scroll", function);
        Some(closure)
    } else {
        None
    };

    let observer = observe(&scroll, document);

    INSTANCE.with(|cell| {
        *cell.borrow_mut() = Some(Instance {
            state: scroll,
            handler,
            observer,
        });
    });
}

fn apply(scroll: &Scroll, from: u32) {
    let ordinal_selector = attribute::ordinal().selector();
    let ordinal_name = attribute::ordinal().name();
    let paged = attribute::paged().name();

    let Ok(elements) = scroll.document.query_selector_all(&ordinal_selector) else {
        return;
    };

    for position in 0..elements.length() {
        let Some(node) = elements.get(position) else {
            continue;
        };
        let element: &web_sys::Element = node.unchecked_ref();
        let ordinal = element
            .get_attribute(ordinal_name)
            .and_then(|value| value.parse::<u32>().ok())
            .unwrap_or(0);

        if ordinal < from {
            continue;
        }

        if ordinal < scroll.revealed {
            let _ = element.remove_attribute(paged);
        } else {
            let _ = element.set_attribute(paged, "");
        }
    }
}

fn advance(scroll: &Rc<RefCell<Scroll>>) {
    let state = scroll.borrow();
    if state.revealed >= state.total {
        return;
    }

    let Some(window) = web_sys::window() else {
        return;
    };

    let Ok(viewport) = window.inner_height() else {
        return;
    };
    let Some(viewport) = viewport.as_f64() else {
        return;
    };

    let Ok(scrolled) = window.scroll_y() else {
        return;
    };

    let Some(body) = state.document.body() else {
        return;
    };
    let document_height = f64::from(body.scroll_height());
    let previous = state.revealed;

    drop(state);

    if scrolled + viewport >= document_height - THRESHOLD {
        let mut state = scroll.borrow_mut();
        let next = (state.revealed + state.capacity).min(state.total);
        state.revealed = next;
        apply(&state, previous);

        let document = state.document.clone();
        let complete = next >= state.total;

        drop(state);

        if complete {
            INSTANCE.with(|cell| {
                if let Some(instance) = cell.borrow_mut().as_mut()
                    && let Some(handler) = instance.handler.take()
                {
                    let function: &js_sys::Function = handler.as_ref().unchecked_ref();
                    let _ = window.remove_event_listener_with_callback("scroll", function);
                }
            });
        }

        progress::show(&document);
        wasm_bindgen_futures::spawn_local(async move {
            let promise = js_sys::Promise::new(&mut |resolve, _| {
                if let Some(window) = web_sys::window() {
                    #[expect(clippy::cast_possible_truncation)]
                    let delay = (proportion::scale(-2) * 1000.0) as i32;
                    let _ = window
                        .set_timeout_with_callback_and_timeout_and_arguments_0(&resolve, delay);
                }
            });
            let _ = JsFuture::from(promise).await;
            progress::hide(&document);
        });
    }
}

fn observe(
    scroll: &Rc<RefCell<Scroll>>,
    document: &Document,
) -> Option<Closure<dyn FnMut(web_sys::Event)>> {
    let search = filter::search().selector();
    let input = document.query_selector(&search).ok()??;

    let weak = Rc::downgrade(scroll);
    let closure = Closure::<dyn FnMut(web_sys::Event)>::new(move |event: web_sys::Event| {
        let Some(scroll) = weak.upgrade() else {
            return;
        };

        let query = event
            .target()
            .and_then(|target| {
                js_sys::Reflect::get(&target, &JsValue::from_str("value"))
                    .ok()
                    .and_then(|value| value.as_string())
            })
            .unwrap_or_default();

        let state = scroll.borrow();

        if query.trim().is_empty() {
            apply(&state, 0);
        } else {
            let ordinals = attribute::ordinal().selector();
            let paged = attribute::paged().name();
            if let Ok(elements) = state.document.query_selector_all(&ordinals) {
                for position in 0..elements.length() {
                    if let Some(node) = elements.get(position) {
                        let element: &web_sys::Element = node.unchecked_ref();
                        let _ = element.remove_attribute(paged);
                    }
                }
            }
        }
    });

    let function: &js_sys::Function = closure.as_ref().unchecked_ref();
    let _ = input.add_event_listener_with_callback("input", function);
    Some(closure)
}
