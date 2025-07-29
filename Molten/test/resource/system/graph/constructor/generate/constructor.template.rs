use component::graph::arena::Valued;
use component::graph::attribute::Attribute;
use component::graph::matrix::Related;
use component::graph::state::Wave;
use std::borrow::Borrow;

fn graph(attribute: Attribute<String>) -> Related<Wave<usize>> {
    let index = Valued::from(attribute.clone());
    system::graph::constructor::relate(&attribute, index.borrow())
}
