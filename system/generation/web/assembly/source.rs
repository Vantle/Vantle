use wasm_bindgen::JsCast;
use web_sys::Document;

pub fn initialize(document: &Document) {
    let prefix = document
        .document_element()
        .and_then(|element| element.get_attribute(root::root().name()))
        .unwrap_or_default();

    let selector = format!(
        "{}[{}]",
        code::block().selector(),
        attribute::source().name()
    );
    let Ok(blocks) = document.query_selector_all(&selector) else {
        return;
    };

    for index in 0..blocks.length() {
        let Some(node) = blocks.get(index) else {
            continue;
        };
        let element: &web_sys::Element = node.unchecked_ref();

        if element
            .query_selector(&code::source().selector())
            .ok()
            .flatten()
            .is_some()
        {
            continue;
        }

        let Some(path) = element.get_attribute(attribute::source().name()) else {
            continue;
        };

        let Some(toolbar) = toolbar(document, element) else {
            continue;
        };

        let Ok(anchor) = document.create_element("a") else {
            continue;
        };
        let _ = anchor.set_attribute("class", &code::source().to_string());
        let _ = anchor.set_attribute("href", &format!("{prefix}{path}"));
        anchor.set_text_content(Some("Source"));

        prepend(&toolbar, &anchor);
    }
}

fn toolbar(document: &Document, element: &web_sys::Element) -> Option<web_sys::Element> {
    if let Some(existing) = element
        .query_selector(&code::toolbar().selector())
        .ok()
        .flatten()
    {
        return Some(existing);
    }

    let created = document.create_element("div").ok()?;
    let _ = created.set_attribute("class", &code::toolbar().to_string());
    prepend(element, &created);
    Some(created)
}

fn prepend(parent: &web_sys::Element, child: &web_sys::Element) {
    if let Some(first) = parent.first_child() {
        let _ = parent.insert_before(child, Some(&first));
    } else {
        let _ = parent.append_child(child);
    }
}
