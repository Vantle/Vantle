use component::graph::arena::Valued;
use component::graph::attribute::Attribute;
use std::path::PathBuf;
use symbolic::constructor::{Constructor, Source};

fn arena(resource: PathBuf) -> Valued<Attribute<String>> {
    Valued::from(utility::unwrap(
        utility::unwrap(Source::path(resource)).module(),
    ))
}
