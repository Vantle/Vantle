use std::cell::RefCell;

use wasm_bindgen::prelude::*;
use web_sys::{Document, IntersectionObserverEntry, IntersectionObserverInit};

thread_local! {
    static OBSERVER: RefCell<Option<web_sys::IntersectionObserver>> = const { RefCell::new(None) };
}

pub fn initialize(document: &Document) {
    observe(document);
    shadow(document);
}

fn observe(document: &Document) {
    OBSERVER.with_borrow(|existing| {
        if let Some(observer) = existing {
            observer.disconnect();
        }
    });

    let callback = Closure::<dyn FnMut(js_sys::Array, JsValue)>::new(
        move |entries: js_sys::Array, _: JsValue| {
            for index in 0..entries.length() {
                let entry: IntersectionObserverEntry = entries.get(index).unchecked_into();
                if entry.is_intersecting() {
                    let _ = entry
                        .target()
                        .set_attribute(attribute::visible().name(), "");
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

    let selector = format!(
        "{}:not({})",
        attribute::animate().selector(),
        attribute::visible().selector()
    );
    let Ok(elements) = document.query_selector_all(&selector) else {
        return;
    };

    for index in 0..elements.length() {
        if let Some(node) = elements.get(index) {
            observer.observe(&node.unchecked_into());
        }
    }

    OBSERVER.with_borrow_mut(|existing| {
        *existing = Some(observer);
    });

    callback.forget();
}

fn shadow(document: &Document) {
    let Ok(Some(nav)) = document.query_selector("nav") else {
        return;
    };

    if nav.get_attribute(attribute::shadow().name()).is_some() {
        return;
    }
    let _ = nav.set_attribute(attribute::shadow().name(), "");

    let scroll = Closure::<dyn FnMut()>::new(move || {
        let Some(window) = web_sys::window() else {
            return;
        };
        let y = window.scroll_y().unwrap_or(0.0);
        if y > 0.0 {
            let _ = nav.set_attribute(attribute::scrolled().name(), "");
        } else {
            let _ = nav.remove_attribute(attribute::scrolled().name());
        }
    });

    if let Some(window) = web_sys::window() {
        let _ = window.add_event_listener_with_callback("scroll", scroll.as_ref().unchecked_ref());
    }
    scroll.forget();
}
