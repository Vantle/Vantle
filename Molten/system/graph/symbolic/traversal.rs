use component::graph::attribute::Attribute;

#[derive(Clone, Copy)]
pub enum Phase {
    Enter,
    Visit,
    Exit,
}

pub struct Context<'a, T> {
    pub node: &'a T,
    pub depth: usize,
    pub parent: Option<&'a T>,
    pub index: usize,
    pub count: usize,
}

pub fn molten<S>(
    ast: &Attribute<String>,
    state: &mut S,
    transform: impl Fn(&mut S, &Context<Attribute<String>>, Phase),
) {
    walk(ast, state, &transform, 0, None, 0, 1);
}

fn walk<S>(
    node: &Attribute<String>,
    state: &mut S,
    transform: &impl Fn(&mut S, &Context<Attribute<String>>, Phase),
    depth: usize,
    parent: Option<&Attribute<String>>,
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
    for (i, child) in node.context.iter().enumerate() {
        walk(
            child,
            state,
            transform,
            depth + 1,
            Some(node),
            i,
            node.context.len(),
        );
    }
    transform(state, &context, Phase::Exit);
}
