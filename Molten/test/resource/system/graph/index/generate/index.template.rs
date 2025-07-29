use component::graph::arena::Valued;

fn index(resource: String) -> Valued<component::graph::attribute::Attribute<String>> {
    let module = utility::resource::attributes::module(resource);
    Valued::from(module)
}
