use component::graph::attribute::{Attribute, Category};
use element::Element;

pub fn molten(ast: &Attribute<String>) -> miette::Result<String> {
    let mut state = State::default();
    token::molten(ast, &mut state, |state, context, phase| {
        color(state, context, phase);
        position(state, context, phase);
        assemble(state, context, phase);
    });
    fragment::fragment(&state.assembler.finish()?)
}

#[derive(Default)]
struct State {
    assembler: assembler::Assembler,
    content: Option<Vec<Element>>,
    node: Option<reference::Reference>,
    prefix: Option<Vec<Element>>,
    suffix: Option<Vec<Element>>,
    separator: Option<Vec<Element>>,
    indent: usize,
}

fn section() -> reference::Reference {
    reference::Reference(&["syntax", "punctuation", "section", "molten"])
}

fn control() -> reference::Reference {
    reference::Reference(&["syntax", "keyword", "control", "molten"])
}

fn token(text: &str, class: reference::Reference) -> Vec<Element> {
    vec![Element::token(class, text)]
}

fn color(state: &mut State, context: &token::Context<Attribute<String>>, phase: token::Phase) {
    if !matches!(phase, token::Phase::Visit) {
        return;
    }
    state.content = match &context.node.category {
        Category::Attribute(value) => Some(token(
            value,
            reference::Reference(&["syntax", "entity", "name", "molten"]),
        )),
        Category::Partition => Some(token(
            ",",
            reference::Reference(&["syntax", "operator", "molten"]),
        )),
        _ => None,
    };
}

fn multiline(node: &Attribute<String>) -> bool {
    node.context
        .iter()
        .any(|c| matches!(c.category, Category::Context | Category::Group))
}

fn position(state: &mut State, context: &token::Context<Attribute<String>>, phase: token::Phase) {
    match phase {
        token::Phase::Enter => {
            state.node = match &context.node.category {
                Category::Attribute(_) => Some(node::attribute()),
                Category::Context => Some(node::context()),
                Category::Group => Some(node::group()),
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
                        state.separator = Some(vec![Element::text(&format!(
                            "\n{}",
                            "    ".repeat(state.indent)
                        ))]);
                    } else {
                        state.separator = Some(vec![Element::text(" ")]);
                    }
                } else if meaningful.is_some() {
                    let previous = &parent.context[context.index - 1];
                    if matches!(previous.category, Category::Void) {
                        state.separator = Some(vec![Element::text(" ")]);
                    } else {
                        state.separator = Some(token(
                            ".",
                            reference::Reference(&["syntax", "punctuation", "accessor", "molten"]),
                        ));
                    }
                }
            }
        }
        token::Phase::Visit => {
            state.prefix = match &context.node.category {
                Category::Context => Some(token("[", control())),
                Category::Group if context.depth > 0 && multiline(context.node) => {
                    let indent = "    ".repeat(state.indent);
                    let mut elements = token("(", section());
                    elements.push(Element::text(&format!("\n{indent}")));
                    Some(elements)
                }
                Category::Group if context.depth > 0 => Some(token("(", section())),
                Category::Attribute(_) if !context.node.context.is_empty() => {
                    let grouped = context.node.context.len() == 1
                        && matches!(context.node.context[0].category, Category::Group);
                    if grouped {
                        None
                    } else {
                        Some(token("(", section()))
                    }
                }
                _ => None,
            };
        }
        token::Phase::Exit => match &context.node.category {
            Category::Group if context.depth > 0 && multiline(context.node) => {
                state.indent -= 1;
                let indent = "    ".repeat(state.indent);
                let mut elements = vec![Element::text(&format!("\n{indent}"))];
                elements.extend(token(")", section()));
                state.suffix = Some(elements);
            }
            Category::Group if context.depth > 0 => {
                state.suffix = Some(token(")", section()));
            }
            Category::Context => {
                state.suffix = Some(token("]", control()));
            }
            Category::Attribute(_) if !context.node.context.is_empty() => {
                let grouped = context.node.context.len() == 1
                    && matches!(context.node.context[0].category, Category::Group);
                if grouped {
                    state.suffix = None;
                } else {
                    state.suffix = Some(token(")", section()));
                }
            }
            _ => {
                state.suffix = None;
            }
        },
    }
}

fn assemble(state: &mut State, context: &token::Context<Attribute<String>>, phase: token::Phase) {
    match phase {
        token::Phase::Enter => {
            if let Some(separator) = state.separator.take() {
                state.assembler.extend(separator);
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
            match &context.node.category {
                Category::Attribute(_) | Category::Context | Category::Group => {
                    let class = state.node.take().unwrap_or(node::attribute());
                    state.assembler.push(Element::labeled(class, children));
                }
                _ => {
                    state.assembler.extend(children);
                }
            }
            if let Some(suffix) = state.suffix.take() {
                state.assembler.extend(suffix);
            }
        }
    }
}
