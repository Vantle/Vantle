use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn start() {
    let Some(window) = web_sys::window() else {
        return;
    };
    let Some(document) = window.document() else {
        return;
    };

    navigate::initialize(&document);

    enhance(&document);

    let document_clone = document.clone();
    let reinitialize = Closure::<dyn FnMut(web_sys::Event)>::new(move |_: web_sys::Event| {
        enhance(&document_clone);
    });
    let _ = document
        .add_event_listener_with_callback("navigate", reinitialize.as_ref().unchecked_ref());
    reinitialize.forget();

    if let Some(html) = document.document_element() {
        let _ = html.class_list().add_1("enhanced");
    }
}

fn enhance(document: &web_sys::Document) {
    theme::initialize(document);
    hamburger::initialize(document);
    clipboard::initialize(document);
    scroll::initialize(document);
    outline::activate(document);
}
