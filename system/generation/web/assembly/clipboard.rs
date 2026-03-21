use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use web_sys::Document;

pub fn initialize(document: &Document) {
    let blocks = document
        .query_selector_all(&code::block().selector())
        .unwrap();

    for index in 0..blocks.length() {
        let Some(block) = blocks
            .item(index)
            .and_then(|n| n.dyn_into::<web_sys::Element>().ok())
        else {
            continue;
        };

        if block
            .query_selector(&button::copy().selector())
            .ok()
            .flatten()
            .is_some()
        {
            continue;
        }

        let toolbar = if let Some(existing) = block
            .query_selector(&code::toolbar().selector())
            .ok()
            .flatten()
        {
            existing
        } else {
            let Ok(created) = document.create_element("div") else {
                continue;
            };
            let _ = created.set_attribute("class", &code::toolbar().to_string());
            let _ = block.append_child(&created);
            created
        };

        let Ok(element) = document.create_element("button") else {
            continue;
        };

        let _ = element.set_attribute("class", &button::copy().to_string());
        element.set_text_content(Some("Copy"));

        let captured = block.clone();
        let handle = element.clone();
        let callback = Closure::<dyn FnMut()>::new(move || {
            let trimmed = {
                let mut content = String::new();
                let children = captured.child_nodes();
                for i in 0..children.length() {
                    if let Some(child) = children.item(i) {
                        let excluded = child.dyn_ref::<web_sys::Element>().is_some_and(|e| {
                            code::toolbar()
                                .words()
                                .iter()
                                .all(|w| e.class_list().contains(w))
                        });
                        if !excluded && let Some(text) = child.text_content() {
                            content.push_str(&text);
                        }
                    }
                }
                content
            };

            let Some(window) = web_sys::window() else {
                return;
            };

            let _ = window.navigator().clipboard().write_text(&trimmed);

            handle.set_text_content(Some("Copied!"));
            let restore = handle.clone();
            let timeout = Closure::<dyn FnMut()>::new(move || {
                restore.set_text_content(Some("Copy"));
            });

            let _ = window.set_timeout_with_callback_and_timeout_and_arguments_0(
                timeout.as_ref().unchecked_ref(),
                2000,
            );
            timeout.forget();
        });

        let _ =
            element.add_event_listener_with_callback("click", callback.as_ref().unchecked_ref());
        callback.forget();

        let _ = toolbar.append_child(&element);
    }
}
