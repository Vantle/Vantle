pub use lava;

use component::graph::matrix::Related;
use component::graph::state::Wave;
use component::graph::traits::node::{Queryable, Translatable};

pub fn deduce(_from: &Wave<usize>, _context: &Related<Wave<usize>>) -> Wave<usize> {
    todo!()
}

pub fn reduce(from: &Wave<usize>, context: &Related<Wave<usize>>) -> Wave<usize> {
    let mut reduction = Wave::default();
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
