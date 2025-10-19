use component::graph::arena::Valued;
use component::graph::attribute::{Attribute, Value};
use component::graph::matrix::Related;
use component::graph::state::Wave;

pub fn relate<T: Value>(
    attribute: &Attribute<T>,
    arena: &Valued<Attribute<T>>,
    relations: &mut Related<Wave<usize>>,
) {
    let subgraph = constructor::relate(attribute, arena);
    for (label, targets) in &subgraph {
        for target in targets {
            relations.relate(label, target);
        }
    }
}
