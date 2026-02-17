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

#[derive(Default)]
struct State {
    output: String,
    content: Option<String>,
    node: Option<&'static str>,
    prefix: Option<String>,
    suffix: Option<String>,
    separator: Option<String>,
    compact: Vec<bool>,
    width: usize,
}

fn color(state: &mut State, context: &token::Context<serde_json::Value>, phase: token::Phase) {
    if !matches!(phase, token::Phase::Visit) {
        return;
    }
    state.content = match context.node {
        serde_json::Value::Null => Some("<span class=\"syntax-constant\">null</span>".into()),
        serde_json::Value::Bool(b) => Some(format!("<span class=\"syntax-constant\">{b}</span>")),
        serde_json::Value::Number(n) => Some(format!("<span class=\"syntax-constant\">{n}</span>")),
        serde_json::Value::String(s) => Some(format!(
            "<span class=\"syntax-string\">\"{}\"</span>",
            escape::escape(s)
        )),
        _ => None,
    };
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

fn position(state: &mut State, context: &token::Context<serde_json::Value>, phase: token::Phase) {
    match phase {
        token::Phase::Enter => {
            state.node = match context.node {
                serde_json::Value::Array(_) => Some("array"),
                serde_json::Value::Object(_) => Some("object"),
                _ => Some("value"),
            };
            let inline = state.compact.last().copied().unwrap_or(false);
            let mut leading = String::new();
            if let Some(serde_json::Value::Object(map)) = context.parent {
                write!(leading, "<span class=\"node-property\">").unwrap();
                if inline {
                    if context.index > 0 {
                        leading.push_str(", ");
                    }
                } else {
                    indent(&mut leading, context.depth);
                }
                if let Some((key, _)) = map.iter().nth(context.index) {
                    write!(
                        leading,
                        "<span class=\"syntax-entity\">\"{}\"</span>",
                        escape::escape(key)
                    )
                    .unwrap();
                }
                leading.push_str("<span class=\"syntax-punctuation\">:</span> ");
            } else if context.parent.is_some() {
                if inline {
                    if context.index > 0 {
                        leading.push_str(", ");
                    }
                } else {
                    indent(&mut leading, context.depth);
                }
            }
            if !leading.is_empty() {
                state.separator = Some(leading);
            }
        }
        token::Phase::Visit => {
            state.prefix = match context.node {
                serde_json::Value::Array(items) if items.is_empty() => {
                    Some("<span class=\"syntax-punctuation\">[]</span>".into())
                }
                serde_json::Value::Array(_) => {
                    let inline = compact(context.node, state.width);
                    state.compact.push(inline);
                    if inline {
                        Some("<span class=\"syntax-punctuation\">[</span>".into())
                    } else {
                        Some("<span class=\"syntax-punctuation\">[</span>\n".into())
                    }
                }
                serde_json::Value::Object(map) if map.is_empty() => {
                    Some("<span class=\"syntax-punctuation\">{}</span>".into())
                }
                serde_json::Value::Object(_) => {
                    let inline = compact(context.node, state.width);
                    state.compact.push(inline);
                    if inline {
                        Some("<span class=\"syntax-punctuation\">{</span>".into())
                    } else {
                        Some("<span class=\"syntax-punctuation\">{</span>\n".into())
                    }
                }
                _ => None,
            };
        }
        token::Phase::Exit => {
            let mut trailing = String::new();
            match context.node {
                serde_json::Value::Array(items) if !items.is_empty() => {
                    if !state.compact.last().copied().unwrap_or(false) {
                        indent(&mut trailing, context.depth);
                    }
                    trailing.push_str("<span class=\"syntax-punctuation\">]</span>");
                    state.compact.pop();
                }
                serde_json::Value::Object(map) if !map.is_empty() => {
                    if !state.compact.last().copied().unwrap_or(false) {
                        indent(&mut trailing, context.depth);
                    }
                    trailing.push_str("<span class=\"syntax-punctuation\">}</span>");
                    state.compact.pop();
                }
                _ => {}
            }
            if context.parent.is_some() {
                let inline = state.compact.last().copied().unwrap_or(false);
                if !inline {
                    if context.index + 1 < context.count {
                        trailing.push_str("<span class=\"syntax-punctuation\">,</span>");
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
