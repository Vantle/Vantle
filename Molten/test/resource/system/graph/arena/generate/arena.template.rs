use component::graph::arena::Valued;
use component::graph::attribute::Attribute;
use std::path::PathBuf;

fn arena(resource: PathBuf) -> Valued<Attribute<String>> {
    Valued::from(utility::resource::attributes::module(resource))
}
