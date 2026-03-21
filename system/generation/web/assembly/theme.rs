use wasm_bindgen::prelude::*;
use web_sys::Document;

pub fn initialize(document: &Document) {
    let Some(html) = document.document_element() else {
        return;
    };

    let Ok(Some(toggle)) = document.query_selector(&button::theme().selector()) else {
        return;
    };

    let prefers = web_sys::window()
        .and_then(|w| w.match_media("(prefers-color-scheme: dark)").ok().flatten())
        .is_some_and(|m| m.matches());

    if let Some(saved) = storage().and_then(|s| s.get_item("theme").ok().flatten()) {
        let _ = html.set_attribute(attribute::theme().name(), &saved);
    }

    update(&html, &toggle, prefers);

    let root = html.clone();
    let switch = toggle.clone();
    let callback = Closure::<dyn FnMut()>::new(move || {
        let current = root.get_attribute(attribute::theme().name());
        let dark = current.as_deref() == Some("dark") || (current.is_none() && prefers);

        let theme = if dark { "light" } else { "dark" };
        let _ = root.set_attribute(attribute::theme().name(), theme);

        if let Some(s) = storage() {
            let _ = s.set_item("theme", theme);
        }

        update(&root, &switch, prefers);
    });

    let _ = toggle.add_event_listener_with_callback("click", callback.as_ref().unchecked_ref());
    callback.forget();
}

fn storage() -> Option<web_sys::Storage> {
    web_sys::window()?.local_storage().ok()?
}

const SUN: &str = include_str!("../../../../resource/system/document/sun.svg");
const MOON: &str = include_str!("../../../../resource/system/document/moon.svg");

fn update(html: &web_sys::Element, toggle: &web_sys::Element, prefers: bool) {
    let current = html.get_attribute(attribute::theme().name());
    let dark = current.as_deref() == Some("dark") || (current.is_none() && prefers);

    toggle.set_inner_html(if dark { SUN } else { MOON });
}
