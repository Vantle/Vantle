use component::graph::attribute::Attribute;
use std::path::PathBuf;

fn breadth(resource: PathBuf) -> Vec<Attribute<String>> {
    utility::resource::attributes::attribute(resource)
        .breadth()
        .cloned()
        .collect()
}
