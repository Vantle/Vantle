use body::Body;

#[must_use]
pub fn render(body: Body) -> Body {
    body.division(|d| {
        d.void("input")
            .attribute("type", "text")
            .attribute("placeholder", "Search — !not .and ,or (group)")
            .data(attribute::search(), "")
            .class(dashboard::search())
            .tag("span", |s| s)
            .data(attribute::counter(), "")
            .class(dashboard::counter())
    })
    .class(dashboard::filter())
}
