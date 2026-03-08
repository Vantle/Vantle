pub use autotest;
pub use web;

#[must_use]
pub fn generation(root: &str) -> index::Index {
    index::Index::new(
        root,
        "system/generation/",
        index::Context::Generation,
        "generation",
    )
}

#[must_use]
pub fn extract(root: &str) -> index::Index {
    index::Index::new(
        root,
        "system/generation/extract.html",
        index::Context::Generation,
        "extract",
    )
}
