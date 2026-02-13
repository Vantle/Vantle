use wasm_bindgen::prelude::*;
use web_sys::Document;

pub fn initialize(document: &Document) {
    let Ok(Some(nav)) = document.query_selector("nav") else {
        return;
    };

    let Ok(Some(sidebar)) = document.query_selector(".sidebar") else {
        return;
    };

    let Ok(button) = document.create_element("button") else {
        return;
    };
    let _ = button.set_attribute("class", "hamburger");
    let _ = button.set_attribute("aria-label", "Toggle navigation");
    button.set_text_content(Some("\u{2630}"));

    let sidebar_clone = sidebar.clone();
    let toggle = Closure::<dyn FnMut()>::new(move || {
        let _ = sidebar_clone.class_list().toggle("open");
    });
    let _ = button.add_event_listener_with_callback("click", toggle.as_ref().unchecked_ref());
    toggle.forget();

    let sidebar_clone = sidebar.clone();
    let close = Closure::<dyn FnMut(web_sys::Event)>::new(move |event: web_sys::Event| {
        let Some(target) = event.target() else {
            return;
        };
        let target: web_sys::Element = target.unchecked_into();

        if sidebar_clone.contains(Some(&target.clone().into())) {
            return;
        }
        if target.closest(".hamburger").ok().flatten().is_some() {
            return;
        }

        let _ = sidebar_clone.class_list().remove_1("open");
    });
    let _ = document.add_event_listener_with_callback("click", close.as_ref().unchecked_ref());
    close.forget();

    let sidebar_clone = sidebar.clone();
    let escape = Closure::<dyn FnMut(web_sys::Event)>::new(move |event: web_sys::Event| {
        let key = js_sys::Reflect::get(&event, &JsValue::from_str("key")).unwrap_or_default();
        if key.as_string().as_deref() == Some("Escape") {
            let _ = sidebar_clone.class_list().remove_1("open");
        }
    });
    let _ = document.add_event_listener_with_callback("keydown", escape.as_ref().unchecked_ref());
    escape.forget();

    if let Some(first_child) = nav.first_child() {
        let _ = nav.insert_before(&button, Some(&first_child));
    } else {
        let _ = nav.append_child(&button);
    }
}
