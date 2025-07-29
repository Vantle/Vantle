pub use lava;

use component::graph::matrix::Related;
use component::graph::node::Node;
use component::graph::traits::node::{Queryable, Translatable};

pub fn deduce(
    _from: &Node<Node<usize>>,
    _context: &Related<Node<Node<usize>>>,
) -> Node<Node<usize>> {
    todo!()
}

pub fn reduce(from: &Node<Node<usize>>, context: &Related<Node<Node<usize>>>) -> Node<Node<usize>> {
    let mut reduction = Node::default();
    for (node, dependencies) in context {
        if node.subset(from).is_some() {
            let divergence = from.diverge(node).unwrap_or(from.clone());
            for dependency in dependencies {
                let _advance = divergence.join(dependency).unwrap_or(divergence.clone());
                if let Some(reduced) = reduction.join(dependency) {
                    reduction = reduced;
                }
            }
        }
    }
    reduction
}
