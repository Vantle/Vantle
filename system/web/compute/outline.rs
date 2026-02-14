use std::cell::RefCell;
use std::rc::Rc;

use wasm_bindgen::prelude::*;
use web_sys::{Document, IntersectionObserverEntry, IntersectionObserverInit};

pub fn activate(document: &Document) {
    let Ok(Some(main)) = document.query_selector("main") else {
        return;
    };

    let Ok(headings) = main.query_selector_all("h2[id], h3[id], h4[id]") else {
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
        let element: web_sys::Element = node.unchecked_into();
        let id = element.id();

        if id.is_empty() {
            continue;
        }

        ids.push(id);
    }

    let visible = Rc::new(RefCell::new(vec![false; ids.len()]));
    let visible_clone = Rc::clone(&visible);
    let ids_clone = ids.clone();
    let document_clone = document.clone();

    let callback = Closure::<dyn FnMut(js_sys::Array, JsValue)>::new(
        move |entries: js_sys::Array, _: JsValue| {
            let mut state = visible_clone.borrow_mut();

            for index in 0..entries.length() {
                let entry: IntersectionObserverEntry = entries.get(index).unchecked_into();
                let id = entry.target().id();
                if let Some(position) = ids_clone.iter().position(|i| i == &id) {
                    state[position] = entry.is_intersecting();
                }
            }

            let Ok(links) = document_clone.query_selector_all(".outline a") else {
                return;
            };
            for link_index in 0..links.length() {
                if let Some(link) = links.get(link_index) {
                    let element: web_sys::Element = link.unchecked_into();
                    let _ = element.class_list().remove_1("active");
                }
            }

            if let Some(active_index) = state.iter().position(|&v| v) {
                let selector = format!(".outline a[href=\"#{}\"]", ids_clone[active_index]);
                if let Ok(Some(active)) = document_clone.query_selector(&selector) {
                    let _ = active.class_list().add_1("active");
                }
            }
        },
    );

    let options = IntersectionObserverInit::new();
    options.set_root_margin("-80px 0px -60% 0px");
    options.set_threshold(&JsValue::from_f64(0.0));

    let Ok(observer) = web_sys::IntersectionObserver::new_with_options(
        callback.as_ref().unchecked_ref(),
        &options,
    ) else {
        return;
    };

    for id in &ids {
        if let Some(target) = document.get_element_by_id(id) {
            observer.observe(&target);
        }
    }

    callback.forget();
}
