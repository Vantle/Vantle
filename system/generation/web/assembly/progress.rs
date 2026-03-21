use web_sys::Document;

const BASELINE: &str =
    "position:fixed;top:0;left:0;height:2px;background:var(--accent);z-index:300;";

fn transition(width: &str, opacity: &str) -> String {
    let base = proportion::scale(-3);
    let fast = proportion::scale(-4);
    format!(
        "{BASELINE}transition:width {base}s cubic-bezier(0.22,1,0.36,1),\
         opacity {fast}s ease;width:{width};opacity:{opacity}"
    )
}

pub fn show(document: &Document) {
    let Some(bar) = resolve(document) else {
        return;
    };
    let _ = bar.set_attribute("style", &transition("80%", "1"));
}

pub fn hide(document: &Document) {
    let Some(bar) = resolve(document) else {
        return;
    };
    let micro = proportion::scale(-5);
    let fast = proportion::scale(-4);
    let _ = bar.set_attribute(
        "style",
        &format!(
            "{BASELINE}transition:width {micro}s ease,\
             opacity {fast}s ease {micro}s;width:100%;opacity:0"
        ),
    );
}

fn resolve(document: &Document) -> Option<web_sys::Element> {
    let selector = attribute::progress().selector();
    document
        .query_selector(&selector)
        .ok()
        .flatten()
        .or_else(|| {
            let element = document.create_element("div").ok()?;
            let _ = element.set_attribute(attribute::progress().name(), "");
            let _ = element.set_attribute("style", &transition("0", "0"));
            document.body()?.append_child(&element).ok()?;
            Some(element)
        })
}
