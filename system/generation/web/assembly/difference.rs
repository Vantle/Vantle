use wasm_bindgen::prelude::*;
use web_sys::Document;

pub fn initialize(document: &Document) {
    let selector = marker::marker().selector();
    let Ok(elements) = document.query_selector_all(&selector) else {
        return;
    };

    for index in 0..elements.length() {
        let Some(node) = elements.get(index) else {
            continue;
        };
        let element: &web_sys::Element = node.unchecked_ref();

        let Some(parent) = element.parent_element() else {
            continue;
        };

        if parent.get_attribute(marker::bound().name()).is_some() {
            continue;
        }
        let _ = parent.set_attribute(marker::bound().name(), "");

        let owned = parent.clone();
        let callback = Closure::<dyn FnMut(web_sys::Event)>::new(move |event: web_sys::Event| {
            event.stop_propagation();
            rotate(&owned);
        });
        let _ = parent.add_event_listener_with_callback("click", callback.as_ref().unchecked_ref());
        callback.forget();
    }
}

fn rotate(container: &web_sys::Element) {
    let selector = marker::marker().selector();
    let Ok(children) = container.query_selector_all(&selector) else {
        return;
    };

    let hidden = marker::hidden().name();
    let mut visible = None;
    let mut spans = Vec::new();

    for index in 0..children.length() {
        let Some(node) = children.get(index) else {
            continue;
        };
        let element: web_sys::Element = node.unchecked_ref::<web_sys::Element>().clone();
        if element.get_attribute(hidden).is_none() && visible.is_none() {
            visible = Some(spans.len());
        }
        spans.push(element);
    }

    if spans.len() < 2 {
        return;
    }

    let current = visible.unwrap_or(0);
    let next = (current + 1) % spans.len();

    let _ = spans[current].set_attribute(hidden, "");
    let _ = spans[next].remove_attribute(hidden);
}
