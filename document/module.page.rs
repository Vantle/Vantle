use body::Chain;
use extraction::Query;
use style::Composition;

pub fn page(root: &str) -> page::Result {
    style::layout("Module", "vantle", "module", root, |c| {
        c.title("Module")
            .anchor("./MODULE.bazel", |a| a.class(class::subtitle()))
            .extract(module_source::EXTRACTIONS.one())
    })
}
