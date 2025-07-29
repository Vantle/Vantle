use component::graph::attribute::Attribute;
use std::path::PathBuf;

fn context(resource: PathBuf) -> Attribute<String> {
    utility::resource::attributes::context(resource)
}
