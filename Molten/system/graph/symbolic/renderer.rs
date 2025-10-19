use arena::error::Error;
use arena::Valued as Arena;
use attribute::{Attribute, Category};
use state::Inference;
use traits::attribute::{Categorized, Contextualized};
use traits::node::{Aliased, Valued};

pub fn attribute(
    width: usize,
    arena: &Arena<Attribute<String>>,
    label: &Attribute<String>,
) -> String {
    let mut output = String::new();
    let mut exceeded = false;

    fn render(
        attribute: &Attribute<String>,
        arena: &Arena<Attribute<String>>,
        width: usize,
        output: &mut String,
        exceeded: &mut bool,
    ) {
        match attribute.category() {
            Category::Attribute(value) => {
                if *exceeded || output.len() > width {
                    *exceeded = true;
                    if let Ok(index) = arena.alias(attribute) {
                        output.push_str(&index.to_string());
                    } else {
                        output.push_str(value);
                    }
                } else {
                    output.push_str(value);
                }

                let context = attribute.context();
                if !context.is_empty() {
                    let single_group =
                        context.len() == 1 && matches!(context[0].category(), Category::Group);

                    if !single_group {
                        output.push('(');
                    }
                    for (index, ctx) in context.iter().enumerate() {
                        if index > 0 {
                            output.push_str(", ");
                        }
                        render(ctx, arena, width, output, exceeded);
                    }
                    if !single_group {
                        output.push(')');
                    }
                }
            }
            Category::Context => {
                output.push('[');
                let context = attribute.context();
                for (index, ctx) in context.iter().enumerate() {
                    if index > 0 {
                        output.push('.');
                    }
                    render(ctx, arena, width, output, exceeded);
                }
                output.push(']');
            }
            Category::Group => {
                output.push('(');
                let context = attribute.context();
                let mut first = true;
                let mut after_partition = false;
                for ctx in context.iter() {
                    if matches!(ctx.category(), Category::Partition) {
                        after_partition = true;
                    } else {
                        if !first {
                            if after_partition {
                                output.push_str(", ");
                                after_partition = false;
                            } else {
                                output.push('.');
                            }
                        }
                        first = false;
                        render(ctx, arena, width, output, exceeded);
                    }
                }
                output.push(')');
            }
            Category::Partition => {}
            Category::Void => {}
        }
    }

    render(label, arena, width, &mut output, &mut exceeded);
    output
}

pub fn inference(
    arena: &Arena<Attribute<String>>,
    inference: &Inference<usize>,
) -> Result<String, Error> {
    let mut output = String::new();

    for (index, (_, particle)) in inference.entries().enumerate() {
        if index > 0 {
            output.push('\n');
        }

        output.push_str(&format!("{}: {{", index));

        let elements: Vec<_> = particle.elements().collect();
        for (idx, (element_id, count)) in elements.iter().enumerate() {
            if idx > 0 {
                output.push_str(", ");
            }

            let element = arena.value(**element_id)?;
            let rendered = attribute(128, arena, element);
            output.push_str(&rendered);

            if **count > 1 {
                output.push_str(&format!(" × {}", count));
            }
        }
        output.push('}');
    }

    Ok(output)
}
