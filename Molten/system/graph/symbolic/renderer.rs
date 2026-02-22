use observe::trace;

use arena::Aliased;
use attribute::{Categorized, Contextualized};
use component::graph::attribute::{Attribute as Data, Category};
use valued::Valued as Arena;

#[must_use]
#[trace(channels = [core])]
pub fn attribute(width: usize, arena: &Arena<Data<String>>, label: &Data<String>) -> String {
    let mut output = String::new();
    let mut exceeded = false;
    render(label, arena, width, &mut output, &mut exceeded);
    output
}

#[trace(channels = [core])]
fn render(
    attribute: &Data<String>,
    arena: &Arena<Data<String>>,
    width: usize,
    output: &mut String,
    exceeded: &mut bool,
) {
    let context = attribute.context();

    match attribute.category() {
        Category::Attribute(value) => {
            if *exceeded || output.len() > width {
                *exceeded = true;
                let text = arena
                    .alias(attribute)
                    .map_or_else(|_| value.clone(), |index| index.to_string());
                output.push_str(&text);
            } else {
                output.push_str(value);
            }

            if !context.is_empty() {
                let grouped =
                    context.len() == 1 && matches!(context[0].category(), Category::Group);

                if grouped {
                    render(&context[0], arena, width, output, exceeded);
                } else {
                    output.push('(');
                    for ctx in context {
                        match ctx.category() {
                            Category::Partition => {
                                output.push_str(", ");
                            }
                            _ => {
                                render(ctx, arena, width, output, exceeded);
                            }
                        }
                    }
                    output.push(')');
                }
            }
        }
        Category::Context => {
            output.push('[');
            let mut first = true;
            let mut voided = false;
            for ctx in context {
                if ctx.category() == &Category::Void {
                    render(ctx, arena, width, output, exceeded);
                    voided = true;
                } else {
                    if !first && !voided {
                        output.push('.');
                    }
                    first = false;
                    voided = false;
                    render(ctx, arena, width, output, exceeded);
                }
            }
            output.push(']');
        }
        Category::Group => {
            output.push('(');
            let mut partitioned = false;
            let mut voided = false;
            for (i, child) in context.iter().enumerate() {
                match child.category() {
                    Category::Partition => {
                        output.push_str(", ");
                        partitioned = true;
                        voided = false;
                    }
                    Category::Void => {
                        render(child, arena, width, output, exceeded);
                        voided = true;
                        partitioned = false;
                    }
                    _ => {
                        if i > 0 && !partitioned && !voided {
                            output.push('.');
                        }
                        partitioned = false;
                        voided = false;
                        render(child, arena, width, output, exceeded);
                    }
                }
            }
            output.push(')');
        }
        Category::Partition => {}
        Category::Void => {
            output.push(' ');
        }
    }
}
