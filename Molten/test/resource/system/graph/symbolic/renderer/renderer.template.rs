use component::graph::attribute::Attribute;
use component::graph::symbolic::constructor::Source;
use std::path::PathBuf;
use symbolic::constructor::Constructor;
use system::graph::attribute::Arena as _;
use valued::Valued as Arena;

fn attribute(resource: PathBuf, width: usize) -> String {
    let label = utility::unwrap(utility::unwrap(Source::path(resource)).module());
    let component: Attribute<String> = label;
    let arena: Arena<Attribute<String>> = utility::unwrap(component.arena());
    symbolic::renderer::attribute(width, &arena, &component)
}
