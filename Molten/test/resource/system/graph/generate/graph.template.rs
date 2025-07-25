use component::graph::arena::Valued;
use component::graph::matrix::Related;
use component::graph::node::Node;
use std::borrow::Borrow;

fn relations(resource: String) -> Related<Node<Node<usize>>> {
    let module = utility::resource::attributes::module(resource);
    let index = Valued::from(module.clone());
    system::graph::constructor::relate(&module, index.borrow())
}
