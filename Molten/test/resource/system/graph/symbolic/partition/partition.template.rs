use component::graph::attribute::Attribute;
use component::graph::symbolic::constructor::Source;
use std::path::PathBuf;
use symbolic::constructor::Constructor;

fn partition(resource: PathBuf) -> Attribute<String> {
    utility::unwrap(utility::unwrap(Source::path(resource)).partition())
}
