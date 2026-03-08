use body::Chain;
use extraction::Query;
use navigation::Composition;

pub fn page(root: &str) -> page::Result {
    navigation::layout("Module", &index::module(root), root, |c| {
        c.title("Module")
            .anchor("./MODULE.bazel", |a| a.class(class::subtitle()))
            .extract(module_source::EXTRACTIONS.one())
    })
}
