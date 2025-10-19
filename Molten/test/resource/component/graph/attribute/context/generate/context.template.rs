use component::graph::attribute::Attribute;
use std::path::PathBuf;
use symbolic::constructor::{Constructor, Source};

fn context(resource: PathBuf) -> Attribute<String> {
    utility::unwrap(utility::unwrap(Source::path(resource)).context())
}
