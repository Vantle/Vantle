use component::graph::attribute::Attribute;
use std::path::PathBuf;

fn attribute(resource: PathBuf) -> Attribute<String> {
    utility::resource::attributes::attribute(resource)
}
