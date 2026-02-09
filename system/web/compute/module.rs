use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn start() {
    let Some(window) = web_sys::window() else {
        return;
    };
    let Some(document) = window.document() else {
        return;
    };

    if let Some(html) = document.document_element() {
        let _ = html.class_list().add_1("enhanced");
    }

    scroll::initialize(&document);
    theme::initialize(&document);
    clipboard::initialize(&document);
}
