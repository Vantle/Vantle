use wasm_bindgen::prelude::*;
use web_sys::{Document, IntersectionObserverEntry, IntersectionObserverInit};

pub fn initialize(document: &Document) {
    let Ok(Some(container)) = document.query_selector(".outline") else {
        return;
    };

    let Ok(Some(main)) = document.query_selector("main") else {
        return;
    };

    let Ok(headings) = main.query_selector_all("h2, h3, h4") else {
        return;
    };

    if headings.length() == 0 {
        return;
    }

    let Ok(label) = document.create_element("div") else {
        return;
    };
    let _ = label.set_attribute("class", "outline-label");
    label.set_text_content(Some("Contents"));
    let _ = container.append_child(&label);

    let mut anchors = Vec::new();

    for index in 0..headings.length() {
        let Some(node) = headings.get(index) else {
            continue;
        };
        let element: web_sys::Element = node.unchecked_into();
        let text = element.text_content().unwrap_or_default();

        if text.is_empty() {
            continue;
        }

        let id = if element.id().is_empty() {
            let generated = text
                .to_lowercase()
                .chars()
                .map(|c| if c.is_alphanumeric() { c } else { '-' })
                .collect::<String>();
            element.set_id(&generated);
            generated
        } else {
            element.id()
        };

        let depth = element.tag_name();

        let Ok(anchor) = document.create_element("a") else {
            continue;
        };
        let _ = anchor.set_attribute("href", &format!("#{id}"));
        let _ = anchor.set_attribute(
            "data-depth",
            match depth.as_str() {
                "H2" => "2",
                "H3" => "3",
                _ => "4",
            },
        );
        anchor.set_text_content(Some(&text));

        let _ = container.append_child(&anchor);
        anchors.push(id);
    }

    let document_clone = document.clone();
    let callback = Closure::<dyn FnMut(js_sys::Array, JsValue)>::new(
        move |entries: js_sys::Array, _: JsValue| {
            for index in 0..entries.length() {
                let entry: IntersectionObserverEntry = entries.get(index).unchecked_into();
                if entry.is_intersecting() {
                    let target = entry.target();
                    let id = target.id();

                    let Ok(links) = document_clone.query_selector_all(".outline a") else {
                        return;
                    };
                    for link_index in 0..links.length() {
                        if let Some(link) = links.get(link_index) {
                            let link_element: web_sys::Element = link.unchecked_into();
                            let _ = link_element.class_list().remove_1("active");
                        }
                    }

                    let selector = format!(".outline a[href=\"#{id}\"]");
                    if let Ok(Some(active)) = document_clone.query_selector(&selector) {
                        let _ = active.class_list().add_1("active");
                    }
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

    for id in &anchors {
        if let Some(target) = document.get_element_by_id(id) {
            observer.observe(&target);
        }
    }

    callback.forget();
}
