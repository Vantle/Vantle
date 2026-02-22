use std::cell::RefCell;
use std::rc::Rc;

use wasm_bindgen::prelude::*;
use web_sys::Document;

pub fn activate(document: &Document) {
    let Ok(Some(main)) = document.query_selector("main") else {
        return;
    };

    let Ok(headings) = main.query_selector_all("h1[id], h2[id], h3[id], h4[id], h5[id], h6[id]")
    else {
        return;
    };

    if headings.length() == 0 {
        return;
    }

    let mut ids = Vec::new();

    for index in 0..headings.length() {
        let Some(node) = headings.get(index) else {
            continue;
        };
        let element: web_sys::HtmlElement = node.unchecked_into();
        let id = element.id();

        if id.is_empty() {
            continue;
        }

        ids.push(id);
    }

    let last_active = Rc::new(RefCell::new(0usize));
    let last_active_clone = Rc::clone(&last_active);
    let ids_clone = ids.clone();
    let document_clone = document.clone();

    let Some(window) = web_sys::window() else {
        return;
    };

    let window_clone = window.clone();

    let callback = Closure::<dyn FnMut()>::new(move || {
        let scroll = window_clone.scroll_y().unwrap_or(0.0);
        let offset = 100.0;

        let mut active_index = 0;

        for (index, id) in ids_clone.iter().enumerate() {
            let Some(element) = document_clone.get_element_by_id(id) else {
                continue;
            };
            let heading: web_sys::HtmlElement = element.unchecked_into();
            let top = f64::from(heading.offset_top());

            if top <= scroll + offset {
                active_index = index;
            }
        }

        let previous = *last_active_clone.borrow();
        if active_index == previous {
            return;
        }
        *last_active_clone.borrow_mut() = active_index;

        let Ok(links) = document_clone.query_selector_all(".outline a") else {
            return;
        };
        for link_index in 0..links.length() {
            if let Some(link) = links.get(link_index) {
                let element: web_sys::Element = link.unchecked_into();
                let _ = element.class_list().remove_1("active");
            }
        }

        let escaped = ids_clone[active_index]
            .replace('\\', "\\\\")
            .replace('"', "\\\"");
        let selector = format!(".outline a[href=\"#{escaped}\"]");
        if let Ok(Some(active)) = document_clone.query_selector(&selector) {
            let _ = active.class_list().add_1("active");
        }
    });

    let _ = window.add_event_listener_with_callback("scroll", callback.as_ref().unchecked_ref());

    callback.forget();

    if let Some(first_id) = ids.first() {
        let escaped = first_id.replace('\\', "\\\\").replace('"', "\\\"");
        let selector = format!(".outline a[href=\"#{escaped}\"]");
        if let Ok(Some(active)) = document.query_selector(&selector) {
            let _ = active.class_list().add_1("active");
        }
    }
}
