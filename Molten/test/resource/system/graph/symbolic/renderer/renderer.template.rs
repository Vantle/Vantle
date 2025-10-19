use component::graph::arena::Valued;
use std::path::PathBuf;
use symbolic::constructor::{Constructor, Source};

fn attribute(resource: PathBuf, width: usize) -> String {
    let label = utility::unwrap(utility::unwrap(Source::path(resource)).module());
    let arena = Valued::from(label.clone());
    symbolic::renderer::attribute(width, &arena, &label)
}
