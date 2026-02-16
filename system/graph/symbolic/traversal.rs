use token::{Context, Phase};

pub fn json<S>(
    value: &serde_json::Value,
    state: &mut S,
    transform: impl Fn(&mut S, &Context<serde_json::Value>, Phase),
) {
    walk(value, state, &transform, 0, None, 0, 1);
}

fn walk<S>(
    node: &serde_json::Value,
    state: &mut S,
    transform: &impl Fn(&mut S, &Context<serde_json::Value>, Phase),
    depth: usize,
    parent: Option<&serde_json::Value>,
    index: usize,
    count: usize,
) {
    let context = Context {
        node,
        depth,
        parent,
        index,
        count,
    };
    transform(state, &context, Phase::Enter);
    transform(state, &context, Phase::Visit);
    match node {
        serde_json::Value::Array(items) => {
            for (i, item) in items.iter().enumerate() {
                walk(
                    item,
                    state,
                    transform,
                    depth + 1,
                    Some(node),
                    i,
                    items.len(),
                );
            }
        }
        serde_json::Value::Object(map) => {
            for (i, (_, value)) in map.iter().enumerate() {
                walk(value, state, transform, depth + 1, Some(node), i, map.len());
            }
        }
        _ => {}
    }
    transform(state, &context, Phase::Exit);
}
