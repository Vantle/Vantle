use observe::trace;

use arena::Valued;
use component::graph::attribute::{Attribute, Value};
use component::graph::relation::Related;
use component::graph::state::wave::Wave;
use relation::Relate;

type Result<T> = std::result::Result<T, arena::error::Error>;

#[trace(channels = [core])]
pub fn relate<T: Value>(
    attribute: &Attribute<T>,
    arena: &Valued<Attribute<T>>,
    relations: &mut Related<Wave<usize>>,
) -> Result<()> {
    let subgraph = constructor::relate(attribute, arena)?;
    for (label, targets) in &subgraph {
        for target in targets {
            relations.relate(label, target);
        }
    }
    Ok(())
}
