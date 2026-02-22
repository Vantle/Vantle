use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::Document;

pub fn initialize(document: &Document) {
    let Some(window) = web_sys::window() else {
        return;
    };

    let document_clone = document.clone();
    let click = Closure::<dyn FnMut(web_sys::Event)>::new(move |event: web_sys::Event| {
        let Some(target) = event.target() else {
            return;
        };
        let element: web_sys::Element = target.unchecked_into();
        let Some(anchor) = element.closest("a").ok().flatten() else {
            return;
        };

        let Some(href) = anchor.get_attribute("href") else {
            return;
        };

        if href.starts_with("http://") || href.starts_with("https://") || href.starts_with('#') {
            return;
        }

        event.prevent_default();
        navigate(&document_clone, &href);
    });

    let _ = document.add_event_listener_with_callback("click", click.as_ref().unchecked_ref());
    click.forget();

    let document_clone = document.clone();
    let popstate = Closure::<dyn FnMut(web_sys::Event)>::new(move |_: web_sys::Event| {
        let Some(window) = web_sys::window() else {
            return;
        };
        let Ok(href) = window.location().href() else {
            return;
        };
        fetch_and_swap(&document_clone, &href, false);
    });

    let _ = window.add_event_listener_with_callback("popstate", popstate.as_ref().unchecked_ref());
    popstate.forget();
}

fn navigate(document: &Document, href: &str) {
    fetch_and_swap(document, href, true);
}

fn same_origin(href: &str) -> bool {
    if href.starts_with('/') || href.starts_with('.') || href.starts_with('#') {
        return true;
    }
    if href.starts_with("http://") || href.starts_with("https://") {
        let Some(window) = web_sys::window() else {
            return false;
        };
        let origin = window.location().origin().unwrap_or_default();
        return href.starts_with(&origin);
    }
    !href.contains("://")
}

fn fetch_and_swap(document: &Document, href: &str, push: bool) {
    if !same_origin(href) {
        if let Some(window) = web_sys::window() {
            let _ = window.location().set_href(href);
        }
        return;
    }

    let document = document.clone();
    let href = href.to_owned();

    wasm_bindgen_futures::spawn_local(async move {
        let Some(window) = web_sys::window() else {
            return;
        };

        let Ok(response) = JsFuture::from(window.fetch_with_str(&href)).await else {
            let _ = window.location().set_href(&href);
            return;
        };

        let response: web_sys::Response = response.unchecked_into();
        if !response.ok() {
            let _ = window.location().set_href(&href);
            return;
        }

        let Ok(text) = JsFuture::from(response.text().unwrap()).await else {
            let _ = window.location().set_href(&href);
            return;
        };

        let Some(text) = text.as_string() else {
            let _ = window.location().set_href(&href);
            return;
        };

        let Ok(parser) = web_sys::DomParser::new() else {
            let _ = window.location().set_href(&href);
            return;
        };

        let Ok(parsed) = parser.parse_from_string(&text, web_sys::SupportedType::TextHtml) else {
            let _ = window.location().set_href(&href);
            return;
        };

        if push {
            let _ = window
                .history()
                .ok()
                .map(|h| h.push_state_with_url(&JsValue::NULL, "", Some(&href)));
        }

        swap(&document, &parsed);

        let scrolled = href
            .rfind('#')
            .and_then(|index| document.get_element_by_id(&href[index + 1..]))
            .map(|element| element.scroll_into_view());

        if scrolled.is_none() {
            window.scroll_to_with_x_and_y(0.0, 0.0);
        }

        let event = web_sys::Event::new("navigate").unwrap();
        let _ = document.dispatch_event(&event);
    });
}

fn swap(document: &Document, parsed: &Document) {
    update_nav(document, parsed);

    for selector in &["main", ".sidebar", ".outline"] {
        if let (Ok(Some(source)), Ok(Some(target))) = (
            parsed.query_selector(selector),
            document.query_selector(selector),
        ) {
            target.set_inner_html(&source.inner_html());
        }
    }

    if let Ok(Some(title)) = parsed.query_selector("title") {
        document.set_title(&title.text_content().unwrap_or_default());
    }

    if let Some(source) = parsed.document_element()
        && let Some(target) = document.document_element()
    {
        for attribute in &["data-context", "data-page", "data-root"] {
            if let Some(value) = source.get_attribute(attribute) {
                let _ = target.set_attribute(attribute, &value);
            }
        }
    }
}

fn update_nav(document: &Document, parsed: &Document) {
    let (Ok(Some(source_nav)), Ok(Some(target_nav))) =
        (parsed.query_selector("nav"), document.query_selector("nav"))
    else {
        return;
    };

    if let (Ok(Some(source)), Ok(Some(target))) = (
        source_nav.query_selector(".nav-logo"),
        target_nav.query_selector(".nav-logo"),
    ) {
        if let Some(href) = source.get_attribute("href") {
            let _ = target.set_attribute("href", &href);
        }
        target.set_inner_html(&source.inner_html());
    }

    let old_root = document
        .document_element()
        .and_then(|e| e.get_attribute("data-root"))
        .unwrap_or_default();
    let new_root = parsed
        .document_element()
        .and_then(|e| e.get_attribute("data-root"))
        .unwrap_or_default();

    if old_root == new_root {
        return;
    }

    let Ok(links) = target_nav.query_selector_all("a:not(.nav-logo)") else {
        return;
    };
    for index in 0..links.length() {
        let Some(node) = links.get(index) else {
            continue;
        };
        let link: web_sys::Element = node.unchecked_into();
        if let Some(href) = link.get_attribute("href")
            && let Some(path) = href.strip_prefix(&old_root)
        {
            let _ = link.set_attribute("href", &format!("{new_root}{path}"));
        }
    }
}
