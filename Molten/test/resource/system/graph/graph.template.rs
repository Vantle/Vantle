use component::graph::attribute::Attribute;
use component::graph::relation::Related;
use component::graph::state::wave::Wave;
use component::graph::symbolic::constructor::Source;
use std::borrow::Borrow;
use std::path::PathBuf;
use symbolic::constructor::Constructor;
use system::graph::attribute::Arena as _;
use valued::Valued as Arena;

fn relations(resource: PathBuf) -> Related<Wave<usize>> {
    let module = utility::unwrap(utility::unwrap(Source::path(resource)).module());
    let index: Arena<Attribute<String>> = utility::unwrap(module.arena());
    utility::unwrap(system::graph::constructor::relate(&module, index.borrow()))
}
