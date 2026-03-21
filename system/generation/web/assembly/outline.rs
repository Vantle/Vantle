use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use web_sys::Document;

fn top(element: &web_sys::Element) -> f64 {
    js_sys::Reflect::get(element, &JsValue::from_str("getBoundingClientRect"))
        .ok()
        .and_then(|method| method.dyn_ref::<js_sys::Function>().cloned())
        .and_then(|method| method.call0(element).ok())
        .and_then(|rect| {
            js_sys::Reflect::get(&rect, &JsValue::from_str("top"))
                .ok()
                .and_then(|value| value.as_f64())
        })
        .unwrap_or(0.0)
}

fn selector() -> String {
    format!("{} a", reference::outline().selector())
}

fn refresh(document: &Document, selector: &str) {
    let Ok(Some(main)) = document.query_selector("main") else {
        return;
    };

    let Ok(headings) = main.query_selector_all("h1[id], h2[id], h3[id], h4[id], h5[id], h6[id]")
    else {
        return;
    };

    let offset = 100.0;
    let mut active = None;

    for index in 0..headings.length() {
        let Some(node) = headings.get(index) else {
            continue;
        };
        let element: web_sys::Element = node.unchecked_into();
        let id = element.id();

        if id.is_empty() {
            continue;
        }

        let position = top(&element);

        if position <= offset {
            active = Some(id);
        }
    }

    let Ok(links) = document.query_selector_all(selector) else {
        return;
    };

    let indicator = reference::active().words()[0];

    if active.is_none() && links.length() > 0 {
        if let Some(node) = links.get(0) {
            let element: web_sys::Element = node.unchecked_into();
            let _ = element.class_list().add_1(indicator);
        }
        return;
    }

    let Some(active) = active else {
        return;
    };

    for index in 0..links.length() {
        let Some(node) = links.get(index) else {
            continue;
        };
        let element: web_sys::Element = node.unchecked_into();
        let href = element.get_attribute("href").unwrap_or_default();
        if href.strip_prefix('#') == Some(&active) {
            let _ = element.class_list().add_1(indicator);
        } else {
            let _ = element.class_list().remove_1(indicator);
        }
    }
}

pub fn update(document: &Document) {
    refresh(document, &selector());
}

pub fn initialize(document: &Document) {
    let Some(window) = web_sys::window() else {
        return;
    };

    let selector = selector();
    refresh(document, &selector);

    let document = document.clone();
    let callback = Closure::<dyn FnMut()>::new(move || {
        refresh(&document, &selector);
    });

    let _ = window.add_event_listener_with_callback("scroll", callback.as_ref().unchecked_ref());
    callback.forget();
}
