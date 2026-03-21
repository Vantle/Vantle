use body::Body;
use reference::Reference;

#[must_use]
pub fn render(body: Body, status: Reference, label: &str, count: usize) -> Body {
    body.tag("span", |s| s.text(&format!("{count} {label}")))
        .class(dashboard::badge())
        .class(status)
}
