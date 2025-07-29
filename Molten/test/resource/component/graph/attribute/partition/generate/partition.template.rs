use component::graph::attribute::Attribute;
use std::path::PathBuf;

fn partition(resource: PathBuf) -> Attribute<String> {
    utility::resource::attributes::partition(resource)
}
