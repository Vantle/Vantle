use wasm_bindgen::prelude::*;
use web_sys::{Document, IntersectionObserverEntry, IntersectionObserverInit};

pub fn initialize(document: &Document) {
    let callback = Closure::<dyn FnMut(js_sys::Array, JsValue)>::new(
        move |entries: js_sys::Array, _: JsValue| {
            for index in 0..entries.length() {
                let entry: IntersectionObserverEntry = entries.get(index).unchecked_into();
                if entry.is_intersecting() {
                    let _ = entry.target().set_attribute("data-visible", "");
                }
            }
        },
    );

    let options = IntersectionObserverInit::new();
    options.set_threshold(&JsValue::from_f64(0.15));

    let Ok(observer) = web_sys::IntersectionObserver::new_with_options(
        callback.as_ref().unchecked_ref(),
        &options,
    ) else {
        return;
    };

    let Ok(elements) = document.query_selector_all("[data-animate]") else {
        return;
    };

    for index in 0..elements.length() {
        if let Some(node) = elements.get(index) {
            observer.observe(&node.unchecked_into());
        }
    }

    callback.forget();
}
