use component::graph::attribute::Attribute;
use component::graph::index::Index as Data;
use component::graph::state::wave::Wave;
use system::graph::index::Index;

fn allocate(mut index: Data<String>, attribute: Attribute<String>) -> (usize, Wave<usize>) {
    utility::unwrap(index.allocate(attribute))
}
