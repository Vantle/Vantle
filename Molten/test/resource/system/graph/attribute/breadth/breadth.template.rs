use component::graph::attribute::Attribute;
use component::graph::symbolic::constructor::Source;
use std::path::PathBuf;
use symbolic::constructor::Constructor;
use system::graph::attribute::Contextualized;

fn breadth(resource: PathBuf) -> Vec<Attribute<String>> {
    let component: Attribute<String> =
        utility::unwrap(utility::unwrap(Source::path(resource)).attribute());
    component.breadth().cloned().collect()
}
