use wasm_bindgen::prelude::*;
use web_sys::Document;

pub fn initialize(document: &Document) {
    let Ok(Some(button)) = document.query_selector(&reference::hamburger().selector()) else {
        return;
    };

    let Ok(Some(sidebar)) = document.query_selector(&reference::sidebar().selector()) else {
        return;
    };

    let captured = sidebar.clone();
    let toggle = Closure::<dyn FnMut()>::new(move || {
        let _ = captured.class_list().toggle(reference::open().words()[0]);
    });
    let _ = button.add_event_listener_with_callback("click", toggle.as_ref().unchecked_ref());
    toggle.forget();

    let captured = sidebar.clone();
    let hamburger = reference::hamburger().selector();
    let close = Closure::<dyn FnMut(web_sys::Event)>::new(move |event: web_sys::Event| {
        let Some(target) = event.target() else {
            return;
        };
        let target: web_sys::Element = target.unchecked_into();

        if captured.contains(Some(&target.clone().into())) {
            return;
        }
        if target.closest(&hamburger).ok().flatten().is_some() {
            return;
        }

        let _ = captured.class_list().remove_1(reference::open().words()[0]);
    });
    let _ = document.add_event_listener_with_callback("click", close.as_ref().unchecked_ref());
    close.forget();

    let captured = sidebar.clone();
    let escape = Closure::<dyn FnMut(web_sys::Event)>::new(move |event: web_sys::Event| {
        let key = js_sys::Reflect::get(&event, &JsValue::from_str("key")).unwrap_or_default();
        if key.as_string().as_deref() == Some("Escape") {
            let _ = captured.class_list().remove_1(reference::open().words()[0]);
        }
    });
    let _ = document.add_event_listener_with_callback("keydown", escape.as_ref().unchecked_ref());
    escape.forget();
}
