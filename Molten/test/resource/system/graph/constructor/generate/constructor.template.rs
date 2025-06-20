use component::graph::arena::Valued;
use component::graph::attribute::Attribute;
use component::graph::matrix::Related;
use component::graph::node::Node;
use std::borrow::Borrow;

fn graph(attribute: Attribute<String>) -> Related<Node<Node<usize>>> {
    let index = Valued::from(attribute.clone());
    system::graph::constructor::relate(&attribute, index.borrow())
}
