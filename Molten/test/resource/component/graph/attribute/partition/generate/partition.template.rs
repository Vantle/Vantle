use component::graph::attribute::Attribute;
use std::path::PathBuf;
use symbolic::constructor::{Constructor, Source};

fn partition(resource: PathBuf) -> Attribute<String> {
    utility::unwrap(utility::unwrap(Source::path(resource)).partition())
}
