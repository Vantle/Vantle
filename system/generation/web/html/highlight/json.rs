use std::collections::HashMap;

use element::Element;

fn elements(value: &serde_json::Value, width: usize) -> miette::Result<Vec<Element>> {
    let mut state = State {
        width,
        ..Default::default()
    };
    traversal::json(value, &mut state, |state, context, phase| {
        color(state, context, phase);
        position(state, context, phase);
        assemble(state, context, phase);
    });
    Ok(state.assembler.finish()?)
}

pub fn json(value: &serde_json::Value, width: usize) -> miette::Result<String> {
    fragment::fragment(&elements(value, width)?)
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
        assemble(state, context, phase);
    });
    fragment::fragment(&state.assembler.finish()?)
}

#[derive(Default)]
struct State<'a> {
    assembler: assembler::Assembler,
    content: Option<Vec<Element>>,
    node: Option<reference::Reference>,
    prefix: Option<Vec<Element>>,
    suffix: Option<Vec<Element>>,
    separator: Option<Vec<Element>>,
    compact: Vec<bool>,
    width: usize,
    path: Vec<difference::Segment>,
    divergences: Option<&'a HashMap<&'a [difference::Segment], &'a difference::Divergence>>,
    skip: usize,
    resumed: bool,
}

fn leaf(value: &serde_json::Value) -> Option<Vec<Element>> {
    match value {
        serde_json::Value::Null => Some(vec![Element::token(syntax::constant(), "null")]),
        serde_json::Value::Bool(b) => {
            Some(vec![Element::token(syntax::constant(), &b.to_string())])
        }
        serde_json::Value::Number(n) => {
            Some(vec![Element::token(syntax::constant(), &n.to_string())])
        }
        serde_json::Value::String(s) => {
            Some(vec![Element::token(syntax::string(), &format!("\"{s}\""))])
        }
        _ => None,
    }
}

fn color(state: &mut State, context: &token::Context<serde_json::Value>, phase: token::Phase) {
    if matches!(phase, token::Phase::Visit) {
        state.content = leaf(context.node);
    }
}

fn divergent(expected: Vec<Element>, actual: Vec<Element>) -> Vec<Element> {
    vec![
        Element::decorated(dashboard::actual(), &[marker::marker().name()], actual),
        Element::decorated(
            dashboard::expected(),
            &[marker::marker().name(), marker::hidden().name()],
            expected,
        ),
    ]
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
                let expected = elements(context.node, state.width).unwrap_or_default();
                let actual = elements(&divergence.actual, state.width).unwrap_or_default();
                state.content = Some(divergent(expected, actual));
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
                state.content = Some(divergent(expected, actual));
            }
        }
        token::Phase::Exit => {
            if state.skip > 0 {
                state.skip -= 1;
                if state.skip == 0 {
                    if let Some(content) = state.content.take() {
                        state.assembler.extend(content);
                    }
                    state.resumed = true;
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

fn bracket(value: &serde_json::Value, width: usize, stack: &mut Vec<bool>) -> Option<Vec<Element>> {
    let (open, empty) = match value {
        serde_json::Value::Array(items) => ("[", items.is_empty()),
        serde_json::Value::Object(map) => ("{", map.is_empty()),
        _ => return None,
    };
    if empty {
        let pair = if open == "[" { "[]" } else { "{}" };
        return Some(vec![Element::token(syntax::punctuation(), pair)]);
    }
    let dense = compact(value, width);
    stack.push(dense);
    let mut result = vec![Element::token(syntax::punctuation(), open)];
    if !dense {
        result.push(Element::text("\n"));
    }
    Some(result)
}

fn indent(level: usize) -> Element {
    Element::text(&"    ".repeat(level))
}

fn punctuation(text: &str) -> Element {
    Element::token(syntax::punctuation(), text)
}

fn position(state: &mut State, context: &token::Context<serde_json::Value>, phase: token::Phase) {
    if state.skip > 0 || state.resumed {
        return;
    }
    match phase {
        token::Phase::Enter => {
            state.node = match context.node {
                serde_json::Value::Array(_) => Some(node::array()),
                serde_json::Value::Object(_) => Some(node::object()),
                _ => Some(node::value()),
            };
            let dense = state.compact.last().copied().unwrap_or(false);
            let mut leading = Vec::<Element>::new();
            if context.parent.is_some() {
                if dense && context.index > 0 {
                    leading.push(Element::text(", "));
                } else if !dense {
                    leading.push(indent(context.depth));
                }
                if let Some(serde_json::Value::Object(map)) = context.parent {
                    if let Some((key, _)) = map.iter().nth(context.index) {
                        leading.push(Element::token(syntax::entity(), &format!("\"{key}\"")));
                    }
                    leading.push(punctuation(":"));
                    leading.push(Element::text(" "));
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
            let mut trailing = Vec::<Element>::new();
            if let Some((close, nonempty)) = delimiters(context.node)
                && nonempty
            {
                if !state.compact.last().copied().unwrap_or(false) {
                    trailing.push(indent(context.depth));
                }
                trailing.push(punctuation(close));
                state.compact.pop();
            }
            if context.parent.is_some() {
                let dense = state.compact.last().copied().unwrap_or(false);
                if !dense {
                    if context.index + 1 < context.count {
                        trailing.push(punctuation(","));
                    }
                    trailing.push(Element::text("\n"));
                }
            }
            if !trailing.is_empty() {
                state.suffix = Some(trailing);
            }
        }
    }
}

fn assemble(state: &mut State, context: &token::Context<serde_json::Value>, phase: token::Phase) {
    if state.skip > 0 {
        return;
    }
    if state.resumed {
        state.resumed = false;
        return;
    }
    let property = matches!(context.parent, Some(serde_json::Value::Object(_)));
    match phase {
        token::Phase::Enter => {
            let separator = state.separator.take();
            if property {
                state.assembler.open();
            }
            if let Some(elements) = separator {
                state.assembler.extend(elements);
            }
            state.assembler.open();
        }
        token::Phase::Visit => {
            if let Some(content) = state.content.take() {
                state.assembler.extend(content);
            }
            if let Some(prefix) = state.prefix.take() {
                state.assembler.extend(prefix);
            }
        }
        token::Phase::Exit => {
            let children = state.assembler.close();
            let class = state.node.take().unwrap_or(node::value());
            let wrapped = Element::labeled(class, children);

            state.assembler.push(wrapped);
            if let Some(suffix) = state.suffix.take() {
                state.assembler.extend(suffix);
            }
            if property {
                let properties = state.assembler.close();
                state
                    .assembler
                    .push(Element::labeled(node::property(), properties));
            }
        }
    }
}
