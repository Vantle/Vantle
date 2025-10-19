use component::graph::arena::Valued;
use component::graph::matrix::Related;
use component::graph::state::Wave;
use std::borrow::Borrow;
use std::path::PathBuf;
use symbolic::constructor::{Constructor, Source};

fn relations(resource: PathBuf) -> Related<Wave<usize>> {
    let module = utility::unwrap(utility::unwrap(Source::path(resource)).module());
    let index = Valued::from(module.clone());
    system::graph::constructor::relate(&module, index.borrow())
}
