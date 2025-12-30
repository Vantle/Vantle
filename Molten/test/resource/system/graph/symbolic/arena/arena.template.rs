use component::graph::attribute::Attribute;
use component::graph::symbolic::constructor::Source;
use std::path::PathBuf;
use symbolic::constructor::Constructor;
use system::arena::Valued as Arena;
use system::graph::attribute::Arena as _;

fn arena(resource: PathBuf) -> Arena<Attribute<String>> {
    let module = utility::unwrap(utility::unwrap(Source::path(resource)).module());
    utility::unwrap(module.arena())
}
