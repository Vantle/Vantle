use wasm_bindgen::prelude::*;
use web_sys::Document;

pub fn initialize(document: &Document) {
    let Some(html) = document.document_element() else {
        return;
    };

    let Ok(Some(container)) = document.query_selector(".nav-links") else {
        return;
    };

    if container
        .query_selector(".theme-toggle")
        .ok()
        .flatten()
        .is_some()
    {
        return;
    }

    let Ok(toggle) = document.create_element("button") else {
        return;
    };

    let _ = toggle.set_attribute("class", "theme-toggle");
    let _ = toggle.set_attribute("aria-label", "Toggle theme");

    let prefers = web_sys::window()
        .and_then(|w| w.match_media("(prefers-color-scheme: dark)").ok().flatten())
        .is_some_and(|m| m.matches());

    if let Some(saved) = storage().and_then(|s| s.get_item("theme").ok().flatten()) {
        let _ = html.set_attribute("data-theme", &saved);
    }

    update(&html, &toggle, prefers);

    let html_clone = html.clone();
    let toggle_clone = toggle.clone();
    let callback = Closure::<dyn FnMut()>::new(move || {
        let current = html_clone.get_attribute("data-theme");
        let dark = current.as_deref() == Some("dark") || (current.is_none() && prefers);

        let theme = if dark { "light" } else { "dark" };
        let _ = html_clone.set_attribute("data-theme", theme);

        if let Some(s) = storage() {
            let _ = s.set_item("theme", theme);
        }

        update(&html_clone, &toggle_clone, prefers);
    });

    let _ = toggle.add_event_listener_with_callback("click", callback.as_ref().unchecked_ref());
    callback.forget();

    let _ = container.append_child(&toggle);
}

fn storage() -> Option<web_sys::Storage> {
    web_sys::window()?.local_storage().ok()?
}

fn update(html: &web_sys::Element, toggle: &web_sys::Element, prefers: bool) {
    let current = html.get_attribute("data-theme");
    let dark = current.as_deref() == Some("dark") || (current.is_none() && prefers);

    toggle.set_text_content(Some(if dark { "\u{2600}" } else { "\u{263e}" }));
}
