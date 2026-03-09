use component::graph::attribute::{Attribute, Category};
use traversal::{Context, Phase, molten};

fn phases(ast: Attribute<String>) -> Vec<String> {
    let mut result = Vec::new();
    molten(
        &ast,
        &mut result,
        |state: &mut Vec<String>, context: &Context<Attribute<String>>, phase: Phase| {
            let label = match &context.node.category {
                Category::Attribute(value) => value.clone(),
                Category::Context => "Context".to_string(),
                Category::Group => "Group".to_string(),
                Category::Partition => "Partition".to_string(),
                Category::Void => "Void".to_string(),
            };
            let phase = match phase {
                Phase::Enter => "Enter",
                Phase::Visit => "Visit",
                Phase::Exit => "Exit",
            };
            state.push(format!("{phase}:{label}"));
        },
    );
    result
}

fn depth(ast: Attribute<String>) -> Vec<usize> {
    let mut result = Vec::new();
    molten(
        &ast,
        &mut result,
        |state: &mut Vec<usize>, context: &Context<Attribute<String>>, phase: Phase| {
            if matches!(phase, Phase::Visit) {
                state.push(context.depth);
            }
        },
    );
    result
}
