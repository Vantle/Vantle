use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use web_sys::Document;

pub fn initialize(document: &Document) {
    let blocks = document.get_elements_by_class_name("code-block");

    for index in 0..blocks.length() {
        let Some(block) = blocks.item(index) else {
            continue;
        };

        if block
            .query_selector(".copy-button")
            .ok()
            .flatten()
            .is_some()
        {
            continue;
        }

        let Ok(button) = document.create_element("button") else {
            continue;
        };

        let _ = button.set_attribute("class", "copy-button");
        button.set_text_content(Some("Copy"));

        let block_clone = block.clone();
        let button_clone = button.clone();
        let callback = Closure::<dyn FnMut()>::new(move || {
            let trimmed = {
                let mut content = String::new();
                let children = block_clone.child_nodes();
                for i in 0..children.length() {
                    if let Some(child) = children.item(i) {
                        let excluded = child
                            .dyn_ref::<web_sys::Element>()
                            .is_some_and(|e| e.class_list().contains("copy-button"));
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

            button_clone.set_text_content(Some("Copied!"));
            let restore = button_clone.clone();
            let timeout = Closure::<dyn FnMut()>::new(move || {
                restore.set_text_content(Some("Copy"));
            });

            let _ = window.set_timeout_with_callback_and_timeout_and_arguments_0(
                timeout.as_ref().unchecked_ref(),
                2000,
            );
            timeout.forget();
        });

        let _ = button.add_event_listener_with_callback("click", callback.as_ref().unchecked_ref());
        callback.forget();

        let _ = block.append_child(&button);
    }
}
