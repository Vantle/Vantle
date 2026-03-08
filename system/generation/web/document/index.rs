#[must_use]
pub fn web(root: &str) -> index::Index {
    index::Index::new(
        root,
        "system/generation/web/",
        index::Context::Generation,
        "web",
    )
}
