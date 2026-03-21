use body::Body;
use element::Location;
use serde_json::Value;

#[must_use]
pub fn render(
    body: Body,
    index: usize,
    parameters: &Value,
    returns: &Value,
    unexpected: Option<&Value>,
    output: Option<&str>,
    reference: &str,
) -> Body {
    let status = if unexpected.is_some() {
        dashboard::fail()
    } else {
        dashboard::pass()
    };

    let input = serde_json::to_string_pretty(parameters).unwrap_or_default();
    let location = || Location {
        source: reference.to_string(),
        start: 0,
        end: 0,
    };

    body.division(|d| {
        d.span(|s| s.text(&format!("#{index}")))
            .class(dashboard::badge())
            .class(status)
            .division(|row| row.located(&input, language::Language::Json, location()))
            .division(|row| match output {
                Some(highlighted) => row
                    .division(|block| block.html(highlighted))
                    .class(code::block())
                    .data(attribute::language(), language::Language::Json.name())
                    .data(attribute::source(), reference),
                None => row.located(
                    &serde_json::to_string_pretty(returns).unwrap_or_default(),
                    language::Language::Json,
                    location(),
                ),
            })
    })
    .class(dashboard::detail())
}
