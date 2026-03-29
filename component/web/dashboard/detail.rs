use body::Body;
use element::Location;
use serde_json::Value;

pub struct Reference<'a> {
    pub input: &'a str,
    pub output: &'a str,
}

#[must_use]
pub fn render(
    body: Body,
    index: usize,
    parameters: &Value,
    returns: &Value,
    unexpected: Option<&Value>,
    highlighted: Option<&str>,
    reference: &Reference<'_>,
) -> Body {
    let status = if unexpected.is_some() {
        dashboard::fail()
    } else {
        dashboard::pass()
    };

    let formatted = serde_json::to_string_pretty(parameters).unwrap_or_default();
    let source = Location {
        source: reference.input.to_string(),
        start: 0,
        end: 0,
    };
    let execution = Location {
        source: reference.output.to_string(),
        start: 0,
        end: 0,
    };

    body.division(|d| {
        d.span(|s| s.text(&format!("#{index}")))
            .class(dashboard::badge())
            .class(status)
            .division(|row| row.located(&formatted, language::Language::Json, source))
            .division(|row| match (highlighted, unexpected) {
                (Some(diff), Some(_)) => row
                    .division(|block| block.html(diff))
                    .class(code::block())
                    .data(attribute::language(), language::Language::Json.name())
                    .data(attribute::source(), reference.output),
                (_, Some(actual)) => row.located(
                    &serde_json::to_string_pretty(actual).unwrap_or_default(),
                    language::Language::Json,
                    execution,
                ),
                _ => row.located(
                    &serde_json::to_string_pretty(returns).unwrap_or_default(),
                    language::Language::Json,
                    execution,
                ),
            })
    })
    .class(dashboard::detail())
}
