use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::Document;

pub fn initialize(document: &Document) {
    let Some(window) = web_sys::window() else {
        return;
    };

    let routed = document.clone();
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

        if href.starts_with("http://")
            || href.starts_with("https://")
            || href.starts_with('#')
            || (!href.to_ascii_lowercase().ends_with(".html") && !href.ends_with('/'))
        {
            return;
        }

        event.prevent_default();
        resolve(&routed, &href, true);
    });

    let _ = document.add_event_listener_with_callback("click", click.as_ref().unchecked_ref());
    click.forget();

    let restored = document.clone();
    let popstate = Closure::<dyn FnMut(web_sys::Event)>::new(move |_: web_sys::Event| {
        let Some(window) = web_sys::window() else {
            return;
        };
        let Ok(href) = window.location().href() else {
            return;
        };
        resolve(&restored, &href, false);
    });

    let _ = window.add_event_listener_with_callback("popstate", popstate.as_ref().unchecked_ref());
    popstate.forget();
}

fn local(href: &str) -> bool {
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

fn resolve(document: &Document, href: &str, push: bool) {
    if !local(href) {
        if let Some(window) = web_sys::window() {
            let _ = window.location().set_href(href);
        }
        return;
    }

    let document = document.clone();
    let href = href.to_owned();

    progress::show(&document);

    wasm_bindgen_futures::spawn_local(async move {
        let Some(window) = web_sys::window() else {
            return;
        };

        let Some(parsed) = fetch(&window, &href).await else {
            progress::hide(&document);
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
        progress::hide(&document);

        if let Some(element) = href
            .rfind('#')
            .and_then(|index| document.get_element_by_id(&href[index + 1..]))
        {
            element.scroll_into_view();
        } else {
            window.scroll_to_with_x_and_y(0.0, 0.0);
        }

        if let Ok(event) = web_sys::Event::new("navigate") {
            let _ = document.dispatch_event(&event);
        }
    });
}

async fn fetch(window: &web_sys::Window, href: &str) -> Option<Document> {
    let response: web_sys::Response = JsFuture::from(window.fetch_with_str(href))
        .await
        .ok()?
        .unchecked_into();
    if !response.ok() {
        return None;
    }
    let text = JsFuture::from(response.text().ok()?)
        .await
        .ok()?
        .as_string()?;
    let parser = web_sys::DomParser::new().ok()?;
    parser
        .parse_from_string(&text, web_sys::SupportedType::TextHtml)
        .ok()
}

fn swap(document: &Document, parsed: &Document) {
    synchronize(document, parsed);

    let container = format!(
        "{}, {}",
        reference::layout().selector(),
        dashboard::frame().selector()
    );
    if let (Ok(Some(source)), Ok(Some(target))) = (
        parsed.query_selector(&container),
        document.query_selector(&container),
    ) {
        target.set_inner_html(&source.inner_html());
        let classes = source.class_name();
        target.set_class_name(&classes);
    }

    if let Ok(Some(title)) = parsed.query_selector("title") {
        document.set_title(&title.text_content().unwrap_or_default());
    }

    if let Some(source) = parsed.document_element()
        && let Some(target) = document.document_element()
    {
        for name in &[
            attribute::context().name(),
            attribute::page().name(),
            attribute::root().name(),
        ] {
            if let Some(value) = source.get_attribute(name) {
                let _ = target.set_attribute(name, &value);
            }
        }
    }
}

fn synchronize(document: &Document, parsed: &Document) {
    let (Ok(Some(incoming)), Ok(Some(existing))) =
        (parsed.query_selector("nav"), document.query_selector("nav"))
    else {
        return;
    };

    let logo = navigation::logo().selector();
    if let (Ok(Some(source)), Ok(Some(target))) = (
        incoming.query_selector(&logo),
        existing.query_selector(&logo),
    ) {
        if let Some(href) = source.get_attribute("href") {
            let _ = target.set_attribute("href", &href);
        }
        target.set_inner_html(&source.inner_html());
    }

    let previous = document
        .document_element()
        .and_then(|e| e.get_attribute(attribute::root().name()))
        .unwrap_or_default();
    let current = parsed
        .document_element()
        .and_then(|e| e.get_attribute(attribute::root().name()))
        .unwrap_or_default();

    if previous == current {
        return;
    }

    let others = format!("a:not({logo})");
    let Ok(links) = existing.query_selector_all(&others) else {
        return;
    };
    for index in 0..links.length() {
        let Some(node) = links.get(index) else {
            continue;
        };
        let link: web_sys::Element = node.unchecked_into();
        if let Some(href) = link.get_attribute("href")
            && let Some(path) = href.strip_prefix(&previous)
        {
            let _ = link.set_attribute("href", &format!("{current}{path}"));
        }
    }
}
