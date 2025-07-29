use component::graph::attribute::Attribute;
use std::path::PathBuf;

fn module(resource: PathBuf) -> Attribute<String> {
    utility::resource::attributes::module(resource)
}
