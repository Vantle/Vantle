use extraction::Query;

#[must_use]
pub fn page(root: &str) -> page::Page {
    navigation::layout("Module", &index::module(root), root, |c| {
        c.title("Module")
            .anchor("./MODULE.bazel", |b| b)
            .class(class::subtitle())
            .extract(module_source::EXTRACTIONS.one())
    })
}
