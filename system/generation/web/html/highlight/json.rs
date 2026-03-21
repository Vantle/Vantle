use std::collections::HashMap;
use std::fmt::Write;

pub fn json(value: &serde_json::Value, width: usize) -> miette::Result<String> {
    let mut state = State {
        width,
        ..Default::default()
    };
    traversal::json(value, &mut state, |state, context, phase| {
        color(state, context, phase);
        position(state, context, phase);
        html(state, context, phase);
    });
    Ok(state.output)
}

pub fn diff(
    expected: &serde_json::Value,
    divergences: &[(Vec<difference::Segment>, difference::Divergence)],
    width: usize,
) -> miette::Result<String> {
    let index = divergences
        .iter()
        .map(|(path, divergence)| (path.as_slice(), divergence))
        .collect::<HashMap<_, _>>();
    let mut state = State {
        width,
        divergences: Some(&index),
        ..Default::default()
    };
    traversal::json(expected, &mut state, |state, context, phase| {
        color(state, context, phase);
        annotate(state, context, phase);
        position(state, context, phase);
        html(state, context, phase);
    });
    Ok(state.output)
}

#[derive(Default)]
struct State<'a> {
    output: String,
    content: Option<String>,
    node: Option<&'static str>,
    prefix: Option<String>,
    suffix: Option<String>,
    separator: Option<String>,
    compact: Vec<bool>,
    width: usize,
    path: Vec<difference::Segment>,
    divergences: Option<&'a HashMap<&'a [difference::Segment], &'a difference::Divergence>>,
    skip: usize,
}

fn leaf(value: &serde_json::Value) -> Option<String> {
    match value {
        serde_json::Value::Null => Some("<span class=\"syntax constant\">null</span>".into()),
        serde_json::Value::Bool(b) => Some(format!("<span class=\"syntax constant\">{b}</span>")),
        serde_json::Value::Number(n) => Some(format!("<span class=\"syntax constant\">{n}</span>")),
        serde_json::Value::String(s) => Some(format!(
            "<span class=\"syntax string\">\"{}\"</span>",
            escape::escape(s)
        )),
        _ => None,
    }
}

fn color(state: &mut State, context: &token::Context<serde_json::Value>, phase: token::Phase) {
    if matches!(phase, token::Phase::Visit) {
        state.content = leaf(context.node);
    }
}

fn divergent(expected_html: &str, actual_html: &str) -> String {
    let marker = marker::marker().name();
    let hidden = marker::hidden().name();
    format!(
        "<span class=\"{}\" {marker}>{}</span>\
         <span class=\"{}\" {marker} {hidden}>{}</span>",
        dashboard::expected(),
        expected_html,
        dashboard::actual(),
        actual_html,
    )
}

fn annotate(state: &mut State, context: &token::Context<serde_json::Value>, phase: token::Phase) {
    let Some(divergences) = state.divergences else {
        return;
    };

    match phase {
        token::Phase::Enter => {
            if state.skip > 0 {
                state.skip += 1;
                return;
            }
            if context.parent.is_some() {
                let segment = match context.parent {
                    Some(serde_json::Value::Object(map)) => map
                        .keys()
                        .nth(context.index)
                        .map(|key| difference::Segment::Key(key.clone())),
                    Some(serde_json::Value::Array(_)) => {
                        Some(difference::Segment::Index(context.index))
                    }
                    _ => None,
                };
                if let Some(segment) = segment {
                    state.path.push(segment);
                }
            }

            if let Some(divergence) = divergences.get(state.path.as_slice()).filter(|_| {
                matches!(
                    context.node,
                    serde_json::Value::Array(_) | serde_json::Value::Object(_)
                )
            }) {
                state.skip = 1;
                let expected_html = json(context.node, state.width).unwrap_or_default();
                let actual_html = json(&divergence.actual, state.width).unwrap_or_default();
                state.content = Some(divergent(&expected_html, &actual_html));
            }
        }
        token::Phase::Visit => {
            if state.skip > 0 {
                return;
            }
            if let Some((expected, actual)) = divergences
                .get(state.path.as_slice())
                .and_then(|d| leaf(context.node).zip(leaf(&d.actual)))
            {
                state.content = Some(divergent(&expected, &actual));
            }
        }
        token::Phase::Exit => {
            if state.skip > 0 {
                state.skip -= 1;
                if state.skip == 0 {
                    if let Some(content) = state.content.take() {
                        state.output.push_str(&content);
                    }
                    state.output.push_str("</span>");
                }
                return;
            }
            if matches!(
                context.parent,
                Some(serde_json::Value::Object(_) | serde_json::Value::Array(_))
            ) {
                state.path.pop();
            }
        }
    }
}

fn compact(value: &serde_json::Value, width: usize) -> bool {
    match value {
        serde_json::Value::Array(items) if items.is_empty() => false,
        serde_json::Value::Object(map) if map.is_empty() => false,
        serde_json::Value::Array(_) | serde_json::Value::Object(_) => {
            serde_json::to_string(value).is_ok_and(|s| s.len() <= width)
        }
        _ => false,
    }
}

fn delimiters(value: &serde_json::Value) -> Option<(&'static str, bool)> {
    match value {
        serde_json::Value::Array(items) => Some(("]", !items.is_empty())),
        serde_json::Value::Object(map) => Some(("}", !map.is_empty())),
        _ => None,
    }
}

fn bracket(value: &serde_json::Value, width: usize, stack: &mut Vec<bool>) -> Option<String> {
    let (open, close, empty) = match value {
        serde_json::Value::Array(items) => ("[", "]", items.is_empty()),
        serde_json::Value::Object(map) => ("{", "}", map.is_empty()),
        _ => return None,
    };
    if empty {
        return Some(format!(
            "<span class=\"syntax punctuation\">{open}{close}</span>"
        ));
    }
    let inline = compact(value, width);
    stack.push(inline);
    let suffix = if inline { "" } else { "\n" };
    Some(format!(
        "<span class=\"syntax punctuation\">{open}</span>{suffix}"
    ))
}

fn position(state: &mut State, context: &token::Context<serde_json::Value>, phase: token::Phase) {
    if state.skip > 0 {
        return;
    }
    match phase {
        token::Phase::Enter => {
            state.node = match context.node {
                serde_json::Value::Array(_) => Some("array"),
                serde_json::Value::Object(_) => Some("object"),
                _ => Some("value"),
            };
            let inline = state.compact.last().copied().unwrap_or(false);
            let mut leading = String::new();
            if context.parent.is_some() {
                if let Some(serde_json::Value::Object(_)) = context.parent {
                    write!(leading, "<span class=\"node-property\">").unwrap();
                }
                if inline && context.index > 0 {
                    leading.push_str(", ");
                } else if !inline {
                    indent(&mut leading, context.depth);
                }
                if let Some(serde_json::Value::Object(map)) = context.parent {
                    if let Some((key, _)) = map.iter().nth(context.index) {
                        write!(
                            leading,
                            "<span class=\"syntax entity\">\"{}\"</span>",
                            escape::escape(key)
                        )
                        .unwrap();
                    }
                    leading.push_str("<span class=\"syntax punctuation\">:</span> ");
                }
            }
            if !leading.is_empty() {
                state.separator = Some(leading);
            }
        }
        token::Phase::Visit => {
            state.prefix = bracket(context.node, state.width, &mut state.compact);
        }
        token::Phase::Exit => {
            let mut trailing = String::new();
            if let Some((close, nonempty)) = delimiters(context.node)
                && nonempty
            {
                if !state.compact.last().copied().unwrap_or(false) {
                    indent(&mut trailing, context.depth);
                }
                write!(
                    trailing,
                    "<span class=\"syntax punctuation\">{close}</span>"
                )
                .unwrap();
                state.compact.pop();
            }
            if context.parent.is_some() {
                let inline = state.compact.last().copied().unwrap_or(false);
                if !inline {
                    if context.index + 1 < context.count {
                        trailing.push_str("<span class=\"syntax punctuation\">,</span>");
                    }
                    trailing.push('\n');
                }
                if matches!(context.parent, Some(serde_json::Value::Object(_))) {
                    trailing.push_str("</span>");
                }
            }
            if !trailing.is_empty() {
                state.suffix = Some(trailing);
            }
        }
    }
}

fn html(state: &mut State, _context: &token::Context<serde_json::Value>, phase: token::Phase) {
    if state.skip > 0 {
        return;
    }
    match phase {
        token::Phase::Enter => {
            if let Some(separator) = state.separator.take() {
                state.output.push_str(&separator);
            }
            if let Some(node) = state.node.take() {
                write!(state.output, "<span class=\"node-{node}\">").unwrap();
            }
        }
        token::Phase::Visit => {
            if let Some(content) = state.content.take() {
                state.output.push_str(&content);
            }
            if let Some(prefix) = state.prefix.take() {
                state.output.push_str(&prefix);
            }
        }
        token::Phase::Exit => {
            if let Some(suffix) = state.suffix.take() {
                state.output.push_str(&suffix);
            }
            state.output.push_str("</span>");
        }
    }
}

fn indent(output: &mut String, level: usize) {
    for _ in 0..level {
        output.push_str("    ");
    }
}
