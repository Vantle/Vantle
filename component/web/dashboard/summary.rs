use body::Body;
use reference::Reference;

#[must_use]
pub fn render(body: Body, label: &str, functions: usize, cases: usize, status: Reference) -> Body {
    body.aside(|a| {
        a.division(|d| d.text(label))
            .class(dashboard::divider())
            .division(|m| {
                m.span(|s| s.text(&functions.to_string()))
                    .text(" functions")
            })
            .class(dashboard::metric())
            .class(status)
            .division(|m| m.span(|s| s.text(&cases.to_string())).text(" cases"))
            .class(dashboard::metric())
            .class(status)
    })
    .class(dashboard::margin())
    .class(status)
}
