use component::graph::attribute::Attribute;
use std::path::PathBuf;
use symbolic::constructor::{Constructor, Source};

fn breadth(resource: PathBuf) -> Vec<Attribute<String>> {
    utility::unwrap(utility::unwrap(Source::path(resource)).attribute())
        .breadth()
        .cloned()
        .collect()
}
