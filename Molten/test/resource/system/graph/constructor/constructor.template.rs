use component::graph::attribute::Attribute;
use component::graph::relation::Related;
use component::graph::state::wave::Wave;
use std::borrow::Borrow;
use system::graph::attribute::Arena as _;
use valued::Valued as Arena;

fn graph(attribute: Attribute<String>) -> Related<Wave<usize>> {
    let index: Arena<Attribute<String>> = utility::unwrap(attribute.arena());
    utility::unwrap(system::graph::constructor::relate(
        &attribute,
        index.borrow(),
    ))
}
