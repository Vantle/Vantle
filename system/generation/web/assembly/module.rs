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
    outline::initialize(&document);

    enhance(&document);

    let callback = {
        let document = document.clone();
        Closure::<dyn FnMut(web_sys::Event)>::new(move |_: web_sys::Event| {
            enhance(&document);
        })
    };
    let _ =
        document.add_event_listener_with_callback("navigate", callback.as_ref().unchecked_ref());
    callback.forget();

    if let Some(html) = document.document_element() {
        let _ = html.class_list().add_1(reference::enhanced().words()[0]);
    }
}

fn enhance(document: &web_sys::Document) {
    theme::initialize(document);
    hamburger::initialize(document);
    clipboard::initialize(document);
    source::initialize(document);
    scroll::initialize(document);
    search::initialize(document);
    paginate::initialize(document);
    expand::initialize(document);
    difference::initialize(document);
    outline::update(document);
}
