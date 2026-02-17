use std::fmt::Write;

use component::graph::attribute::{Attribute, Category};

pub fn molten(ast: &Attribute<String>) -> miette::Result<String> {
    let mut state = State::default();
    token::molten(ast, &mut state, |state, context, phase| {
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
    indent: usize,
}

fn color(state: &mut State, context: &token::Context<Attribute<String>>, phase: token::Phase) {
    if !matches!(phase, token::Phase::Visit) {
        return;
    }
    state.content = match &context.node.category {
        Category::Attribute(value) => Some(format!(
            "<span class=\"syntax-entity syntax-name syntax-molten\">{}</span>",
            escape::escape(value)
        )),
        Category::Partition => {
            Some("<span class=\"syntax-operator syntax-molten\">,</span>".into())
        }
        _ => None,
    };
}

fn multiline(node: &Attribute<String>) -> bool {
    node.context
        .iter()
        .any(|c| matches!(c.category, Category::Context | Category::Group))
}

fn position(state: &mut State, context: &token::Context<Attribute<String>>, phase: token::Phase) {
    let dot = "<span class=\"syntax-punctuation syntax-accessor syntax-molten\">.</span>";
    match phase {
        token::Phase::Enter => {
            state.node = match &context.node.category {
                Category::Attribute(_) => Some("attribute"),
                Category::Context => Some("context"),
                Category::Group => Some("group"),
                _ => None,
            };
            if matches!(context.node.category, Category::Group)
                && context.depth > 0
                && multiline(context.node)
            {
                state.indent += 1;
            }
            if let Some(parent) = context.parent
                && context.index > 0
                && !matches!(context.node.category, Category::Partition | Category::Void)
                && matches!(parent.category, Category::Group | Category::Context)
            {
                let meaningful = parent.context[..context.index]
                    .iter()
                    .rev()
                    .find(|s| !matches!(s.category, Category::Void));

                if meaningful.is_some_and(|m| matches!(m.category, Category::Partition)) {
                    if matches!(parent.category, Category::Group) && multiline(parent) {
                        state.separator = Some(format!("\n{}", "    ".repeat(state.indent)));
                    } else {
                        state.separator = Some(" ".into());
                    }
                } else if meaningful.is_some() {
                    let previous = &parent.context[context.index - 1];
                    if matches!(previous.category, Category::Void) {
                        state.separator = Some(" ".into());
                    } else {
                        state.separator = Some(dot.into());
                    }
                }
            }
        }
        token::Phase::Visit => {
            state.prefix = match &context.node.category {
                Category::Context => Some(
                    "<span class=\"syntax-keyword syntax-control syntax-molten\">[</span>".into(),
                ),
                Category::Group if context.depth > 0 && multiline(context.node) => {
                    let indent = "    ".repeat(state.indent);
                    Some(format!(
                        "<span class=\"syntax-punctuation syntax-section syntax-molten\">(</span>\n{indent}"
                    ))
                }
                Category::Group if context.depth > 0 => Some(
                    "<span class=\"syntax-punctuation syntax-section syntax-molten\">(</span>"
                        .into(),
                ),
                Category::Attribute(_) if !context.node.context.is_empty() => {
                    let grouped = context.node.context.len() == 1
                        && matches!(context.node.context[0].category, Category::Group);
                    if grouped {
                        None
                    } else {
                        Some(
                            "<span class=\"syntax-punctuation syntax-section syntax-molten\">(</span>".into(),
                        )
                    }
                }
                _ => None,
            };
        }
        token::Phase::Exit => match &context.node.category {
            Category::Group if context.depth > 0 && multiline(context.node) => {
                state.indent -= 1;
                let indent = "    ".repeat(state.indent);
                state.suffix = Some(format!(
                    "\n{indent}<span class=\"syntax-punctuation syntax-section syntax-molten\">)</span>"
                ));
            }
            Category::Group if context.depth > 0 => {
                state.suffix = Some(
                    "<span class=\"syntax-punctuation syntax-section syntax-molten\">)</span>"
                        .into(),
                );
            }
            Category::Context => {
                state.suffix = Some(
                    "<span class=\"syntax-keyword syntax-control syntax-molten\">]</span>".into(),
                );
            }
            Category::Attribute(_) if !context.node.context.is_empty() => {
                let grouped = context.node.context.len() == 1
                    && matches!(context.node.context[0].category, Category::Group);
                if grouped {
                    state.suffix = None;
                } else {
                    state.suffix = Some(
                        "<span class=\"syntax-punctuation syntax-section syntax-molten\">)</span>"
                            .into(),
                    );
                }
            }
            _ => {
                state.suffix = None;
            }
        },
    }
}

fn html(state: &mut State, context: &token::Context<Attribute<String>>, phase: token::Phase) {
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
            match &context.node.category {
                Category::Attribute(_) | Category::Context | Category::Group => {
                    state.output.push_str("</span>");
                }
                _ => {}
            }
        }
    }
}
