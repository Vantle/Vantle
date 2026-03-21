use body::Body;
use element::Location;
use serde_json::Value;

pub struct Source {
    pub content: String,
    pub path: String,
    pub start: usize,
    pub end: usize,
}

pub struct Group {
    pub function: String,
    pub tags: Vec<String>,
    pub source: Option<Source>,
    pub reference: String,
    pub cases: Vec<Case>,
}

pub struct Case {
    pub parameters: Value,
    pub returns: Value,
    pub unexpected: Option<Value>,
}

impl Case {
    #[must_use]
    pub fn passed(&self) -> bool {
        self.unexpected.is_none()
    }
}

#[must_use]
pub fn source(body: Body, group: &Group) -> Body {
    match &group.source {
        Some(source) => body.located(
            &source.content,
            language::Language::Rust,
            Location {
                source: source.path.clone(),
                start: source.start,
                end: source.end,
            },
        ),
        None => body,
    }
}

#[must_use]
pub fn cases(
    body: Body,
    group: &Group,
    highlight: impl Fn(&Value, Option<&Value>) -> Option<String>,
) -> Body {
    group.cases.iter().enumerate().fold(body, |c, (i, case)| {
        let output = highlight(&case.returns, case.unexpected.as_ref());
        detail::render(
            c,
            i,
            &case.parameters,
            &case.returns,
            case.unexpected.as_ref(),
            output.as_deref(),
            &group.reference,
        )
    })
}

#[must_use]
pub fn render(body: Body, group: &Group, content: impl FnOnce(Body) -> Body) -> Body {
    let total = group.cases.len();
    let passed = group.cases.iter().filter(|c| c.passed()).count();
    let failed = total - passed;

    let status = if failed > 0 { "fail" } else { "pass" };
    let indicator = if failed > 0 {
        dashboard::fail()
    } else {
        dashboard::pass()
    };

    let joined = std::iter::once(group.function.as_str())
        .chain(group.tags.iter().map(String::as_str))
        .chain(std::iter::once(status))
        .collect::<Vec<_>>()
        .join(" ");

    let ratio = if total > 0 { passed * 100 / total } else { 100 };
    let remainder = 100 - ratio;

    body.section(|s| {
        s.division(|bar| {
            bar.when(failed > 0, |b| {
                b.span(|s| s)
                    .class(dashboard::segment())
                    .class(indicator)
                    .inline(&format!("width: {remainder}%"))
            })
            .span(|s| s)
            .class(dashboard::segment())
            .class(dashboard::pass())
            .inline(&format!("width: {ratio}%"))
        })
        .class(dashboard::bar())
        .details(|details| {
            details
                .summary(|summary| {
                    summary
                        .span(|s| s.text(&group.function))
                        .division(|d| {
                            let d = group
                                .tags
                                .iter()
                                .fold(d, |d, t| d.span(|s| s.text(t)).class(dashboard::tag()));
                            d.span(|s| s.text(&format!("{passed}/{total}")))
                                .class(dashboard::tag())
                                .class(indicator)
                        })
                        .class(dashboard::tags())
                })
                .division(content)
                .class(dashboard::content())
        })
    })
    .class(dashboard::card())
    .data(search::function(), &group.function)
    .data(search::tags(), &joined)
    .data(status::status(), status)
}
