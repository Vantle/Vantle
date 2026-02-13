use web_sys::{Document, Element};

type Entry = (&'static str, String, &'static str);

fn append(document: &Document, sidebar: &Element, page: &str, entries: &[Entry]) {
    for (text, href, identifier) in entries {
        let Ok(anchor) = document.create_element("a") else {
            continue;
        };
        let _ = anchor.set_attribute("href", href);
        anchor.set_text_content(Some(text));
        if *identifier == page {
            let _ = anchor.set_attribute("aria-current", "page");
        }
        let _ = sidebar.append_child(&anchor);
    }
}

pub fn initialize(document: &Document) {
    let Some(html) = document.document_element() else {
        return;
    };

    let context = html.get_attribute("data-context").unwrap_or_default();
    let page = html.get_attribute("data-page").unwrap_or_default();
    let root = html.get_attribute("data-root").unwrap_or_default();

    let Ok(Some(sidebar)) = document.query_selector(".sidebar") else {
        return;
    };

    let (primary, legal): (Vec<Entry>, Vec<Entry>) = match context.as_str() {
        "molten" => (
            vec![("Molten", format!("{root}Molten/"), "readme")],
            vec![
                ("Info", format!("{root}Molten/info.html"), "info"),
                ("Notice", format!("{root}Molten/notice.html"), "notice"),
                ("License", format!("{root}Molten/license.html"), "license"),
            ],
        ),
        _ => (
            vec![
                ("Vantle", format!("{root}index.html"), "readme"),
                ("Module", format!("{root}module.html"), "module"),
            ],
            vec![
                ("Info", format!("{root}info.html"), "info"),
                ("Notice", format!("{root}notice.html"), "notice"),
                ("License", format!("{root}license.html"), "license"),
            ],
        ),
    };

    append(document, &sidebar, &page, &primary);

    if let Ok(label) = document.create_element("div") {
        let _ = label.set_attribute("class", "sidebar-label");
        label.set_text_content(Some("Legal"));
        let _ = sidebar.append_child(&label);
    }

    append(document, &sidebar, &page, &legal);
}
