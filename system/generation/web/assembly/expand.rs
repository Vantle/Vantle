use wasm_bindgen::prelude::*;
use web_sys::Document;

fn expanded() -> String {
    format!(
        "{}{}",
        dashboard::card().selector(),
        attribute::expanded().selector()
    )
}

fn backdrop(document: &Document) -> Option<web_sys::Element> {
    let selector = attribute::backdrop().selector();
    if let Ok(Some(existing)) = document.query_selector(&selector) {
        return Some(existing);
    }

    let element = document.create_element("div").ok()?;
    for word in dashboard::backdrop().words() {
        let _ = element.class_list().add_1(word);
    }
    let _ = element.set_attribute(attribute::backdrop().name(), "");

    let owned = document.clone();
    let callback = Closure::<dyn FnMut(web_sys::Event)>::new(move |event: web_sys::Event| {
        let Some(target) = event.target() else {
            return;
        };
        let target: &web_sys::Element = target.unchecked_ref();
        if target.get_attribute(attribute::backdrop().name()).is_some() {
            collapse(&owned);
        }
    });
    let _ = element.add_event_listener_with_callback("click", callback.as_ref().unchecked_ref());
    callback.forget();

    document.body()?.append_child(&element).ok()?;
    Some(element)
}

fn expand(document: &Document, card: &web_sys::Element) {
    let Some(overlay) = backdrop(document) else {
        return;
    };

    let _ = card.set_attribute(attribute::expanded().name(), "");
    let _ = overlay.set_attribute(attribute::expanded().name(), "");

    if let Some(body) = document.body() {
        let _ = body.set_attribute(attribute::lock().name(), "");
    }
}

fn collapse(document: &Document) {
    let selector = expanded();

    if let Ok(cards) = document.query_selector_all(&selector) {
        for index in 0..cards.length() {
            let Some(node) = cards.get(index) else {
                continue;
            };
            let element: &web_sys::Element = node.unchecked_ref();
            let _ = element.remove_attribute(attribute::expanded().name());
            if let Ok(Some(details)) = element.query_selector("details") {
                let _ = details.remove_attribute("open");
            }
        }
    }

    let selector = attribute::backdrop().selector();
    if let Ok(Some(overlay)) = document.query_selector(&selector) {
        let _ = overlay.remove_attribute(attribute::expanded().name());
    }

    if let Some(body) = document.body() {
        let _ = body.remove_attribute(attribute::lock().name());
    }
}

pub fn initialize(document: &Document) {
    let selector = dashboard::card().selector();
    let Ok(cards) = document.query_selector_all(&selector) else {
        return;
    };

    for index in 0..cards.length() {
        let Some(node) = cards.get(index) else {
            continue;
        };
        let card: web_sys::Element = node.unchecked_ref::<web_sys::Element>().clone();

        if card.get_attribute(attribute::bound().name()).is_some() {
            continue;
        }
        let _ = card.set_attribute(attribute::bound().name(), "");

        let Ok(Some(details)) = card.query_selector("details") else {
            continue;
        };

        let owned = document.clone();
        let inner = details.clone();
        let toggle = Closure::<dyn FnMut(web_sys::Event)>::new(move |_: web_sys::Event| {
            collapse(&owned);
            if inner.get_attribute("open").is_some() {
                expand(&owned, &card);
            }
        });
        let _ = details.add_event_listener_with_callback("toggle", toggle.as_ref().unchecked_ref());
        toggle.forget();
    }

    let bound = document
        .document_element()
        .is_some_and(|html| html.get_attribute(attribute::keyboard().name()).is_some());

    if !bound {
        if let Some(html) = document.document_element() {
            let _ = html.set_attribute(attribute::keyboard().name(), "");
        }

        let keyed = document.clone();
        let keyboard = Closure::<dyn FnMut(web_sys::KeyboardEvent)>::new(
            move |event: web_sys::KeyboardEvent| {
                if event.key() == "Escape"
                    && !editing::active(&event)
                    && keyed.query_selector(&expanded()).ok().flatten().is_some()
                {
                    event.prevent_default();
                    collapse(&keyed);
                }
            },
        );

        let _ =
            document.add_event_listener_with_callback("keydown", keyboard.as_ref().unchecked_ref());
        keyboard.forget();
    }
}
