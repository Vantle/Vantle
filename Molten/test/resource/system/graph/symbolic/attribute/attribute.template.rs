use component::graph::attribute::Attribute;
use component::graph::symbolic::constructor::Source;
use std::path::PathBuf;
use symbolic::constructor::Constructor;

fn attribute(resource: PathBuf) -> Attribute<String> {
    utility::unwrap(utility::unwrap(Source::path(resource)).attribute())
}

fn error(resource: PathBuf) -> bool {
    utility::unwrap(Source::path(resource)).attribute().is_err()
}
