use component::graph::attribute::Attribute;
use component::graph::symbolic::constructor::Source;
use std::path::PathBuf;
use symbolic::constructor::Constructor;

fn group(resource: PathBuf) -> Attribute<String> {
    utility::unwrap(utility::unwrap(Source::path(resource)).group())
}
