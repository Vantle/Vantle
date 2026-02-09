use wasm_bindgen::prelude::*;
use web_sys::Document;

pub fn initialize(document: &Document) {
    let Some(html) = document.document_element() else {
        return;
    };

    let Ok(Some(nav)) = document.query_selector("nav") else {
        return;
    };

    let Ok(toggle) = document.create_element("button") else {
        return;
    };

    let _ = toggle.set_attribute("class", "theme-toggle");
    let _ = toggle.set_attribute("aria-label", "Toggle theme");

    let prefers = web_sys::window()
        .and_then(|w| w.match_media("(prefers-color-scheme: dark)").ok().flatten())
        .is_some_and(|m| m.matches());

    update(&html, &toggle, prefers);

    let html_clone = html.clone();
    let toggle_clone = toggle.clone();
    let callback = Closure::<dyn FnMut()>::new(move || {
        let current = html_clone.get_attribute("data-theme");
        let dark = current.as_deref() == Some("dark") || (current.is_none() && prefers);

        let _ = html_clone.set_attribute("data-theme", if dark { "light" } else { "dark" });

        update(&html_clone, &toggle_clone, prefers);
    });

    let _ = toggle.add_event_listener_with_callback("click", callback.as_ref().unchecked_ref());
    callback.forget();

    let _ = nav.append_child(&toggle);
}

fn update(html: &web_sys::Element, toggle: &web_sys::Element, prefers: bool) {
    let current = html.get_attribute("data-theme");
    let dark = current.as_deref() == Some("dark") || (current.is_none() && prefers);

    toggle.set_text_content(Some(if dark { "\u{2600}" } else { "\u{263e}" }));
}
