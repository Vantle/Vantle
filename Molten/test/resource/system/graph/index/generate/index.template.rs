use component::graph::arena::Valued;
use std::path::PathBuf;
use symbolic::constructor::{Constructor, Source};

fn index(resource: PathBuf) -> Valued<component::graph::attribute::Attribute<String>> {
    let module = utility::unwrap(utility::unwrap(Source::path(resource)).module());
    Valued::from(module)
}
