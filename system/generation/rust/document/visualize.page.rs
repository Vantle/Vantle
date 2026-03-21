use body::Body;
use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize)]
pub struct Execution {
    pub source: Source,
    pub functions: Vec<Function>,
}

#[derive(Deserialize)]
pub struct Source {
    pub file: String,
    pub cases: String,
}

#[derive(Deserialize)]
pub struct Function {
    pub function: String,
    #[serde(default)]
    pub tags: Vec<String>,
    pub cases: Vec<Case>,
}

#[derive(Deserialize)]
pub struct Case {
    pub parameters: Value,
    pub returns: Value,
    pub unexpected: Option<Value>,
}

pub struct Template {
    pub path: String,
    pub content: String,
}

#[must_use]
pub fn cards(execution: Execution, templates: &[Template]) -> Vec<card::Group> {
    let file = execution.source.file;
    let reference = execution.source.cases;
    execution
        .functions
        .into_iter()
        .map(|function| {
            let source = source(templates, &file, &function.function);
            card::Group {
                function: function.function,
                tags: function.tags,
                source,
                reference: reference.clone(),
                cases: function
                    .cases
                    .into_iter()
                    .map(|c| card::Case {
                        parameters: c.parameters,
                        returns: c.returns,
                        unexpected: c.unexpected,
                    })
                    .collect::<Vec<_>>(),
            }
        })
        .collect::<Vec<_>>()
}

const PAGE_SIZE: usize = 38;

#[must_use]
pub fn page(
    root: &str,
    groups: Vec<card::Group>,
    content: impl Fn(Body, &card::Group) -> Body,
) -> page::Page {
    let (failing, passing): (Vec<_>, Vec<_>) = groups
        .into_iter()
        .partition(|g| g.cases.iter().any(|c| !c.passed()));

    let failures = failing.len();
    let successes = passing.len();

    let (passed, failed) =
        failing
            .iter()
            .chain(&passing)
            .flat_map(|g| &g.cases)
            .fold(
                (0, 0),
                |(p, f), c| {
                    if c.passed() { (p + 1, f) } else { (p, f + 1) }
                },
            );

    navigation::dashboard(
        "Visualize",
        &index::generation::autotest::visualize(root),
        root,
        |frame| {
            let frame = summary::render(frame, "Passing", successes, passed, dashboard::pass());
            let frame = frame.main(|c| {
                let c = search::render(c);
                let offset = failing.len();

                c.division(|grid| {
                    let grid = section(grid, "Failing", &failing, 0, &content);
                    let grid = section(grid, "Passing", &passing, offset, &content);
                    grid.division(|d| d.text("No tests match your query"))
                        .class(dashboard::empty())
                        .data(filter::empty(), "")
                        .data(filter::hidden(), "")
                })
                .class(dashboard::grid())
                .data(attribute::capacity(), &PAGE_SIZE.to_string())
            });
            if failures > 0 {
                summary::render(frame, "Failing", failures, failed, dashboard::fail())
            } else {
                frame
            }
        },
    )
}

fn section(
    grid: Body,
    label: &str,
    groups: &[card::Group],
    offset: usize,
    content: &impl Fn(Body, &card::Group) -> Body,
) -> Body {
    if groups.is_empty() {
        return grid;
    }
    let grid = grid.division(|d| d.text(label)).class(dashboard::divider());
    groups.iter().enumerate().fold(grid, |g, (i, group)| {
        card::render(g, group, |body| content(body, group))
            .data(attribute::ordinal(), &(i + offset).to_string())
    })
}

fn source(templates: &[Template], path: &str, function: &str) -> Option<card::Source> {
    let template = templates.iter().find(|t| t.path.ends_with(path))?;
    let query = query(function);
    let extractions = extract::extract(&template.content, &query, language::Language::Rust).ok()?;
    extractions
        .into_iter()
        .next()
        .map(|extraction| card::Source {
            content: extraction.content,
            path: template.path.clone(),
            start: extraction.start,
            end: extraction.end,
        })
}

fn query(function: &str) -> String {
    use std::fmt::Write;

    let segments = function.split('.').collect::<Vec<_>>();
    let name = segments.last().copied().unwrap_or(function);

    if segments.len() <= 1 {
        return format!(
            "(function_item name: (identifier) @name (#eq? @name \"{name}\")) @capture"
        );
    }

    let modules = &segments[..segments.len() - 1];
    let mut result = String::new();
    for module in modules {
        let _ = write!(
            result,
            "(mod_item name: (identifier) @_{module} (#eq? @_{module} \"{module}\") body: (declaration_list "
        );
    }
    let _ = write!(
        result,
        "(function_item name: (identifier) @name (#eq? @name \"{name}\")) @capture"
    );
    result.push_str(&"))".repeat(modules.len()));
    result
}
