use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use web_sys::Document;

pub fn update(document: &Document) {
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

        let rect = js_sys::Reflect::get(&element, &JsValue::from_str("getBoundingClientRect"))
            .ok()
            .and_then(|method| method.dyn_ref::<js_sys::Function>().cloned())
            .and_then(|method| method.call0(&element).ok())
            .unwrap_or_default();
        let top = js_sys::Reflect::get(&rect, &JsValue::from_str("top"))
            .ok()
            .and_then(|value| value.as_f64())
            .unwrap_or(0.0);

        if top <= offset {
            active = Some(id);
        }
    }

    let Ok(links) = document.query_selector_all(".outline a") else {
        return;
    };

    if active.is_none() && links.length() > 0 {
        if let Some(node) = links.get(0) {
            let element: web_sys::Element = node.unchecked_into();
            let _ = element.class_list().add_1("active");
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
            let _ = element.class_list().add_1("active");
        } else {
            let _ = element.class_list().remove_1("active");
        }
    }
}

pub fn initialize(document: &Document) {
    let Some(window) = web_sys::window() else {
        return;
    };

    update(document);

    let document = document.clone();
    let callback = Closure::<dyn FnMut()>::new(move || {
        update(&document);
    });

    let _ = window.add_event_listener_with_callback("scroll", callback.as_ref().unchecked_ref());
    callback.forget();
}
