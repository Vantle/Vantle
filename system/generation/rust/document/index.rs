#[must_use]
pub fn autotest(root: &str) -> index::Index {
    index::Index::new(
        root,
        "system/generation/rust/",
        index::Context::Autotest,
        "autotest",
    )
}

#[must_use]
pub fn function(root: &str) -> index::Index {
    index::Index::new(
        root,
        "system/generation/rust/function.html",
        index::Context::Autotest,
        "function",
    )
}

#[must_use]
pub fn performance(root: &str) -> index::Index {
    index::Index::new(
        root,
        "system/generation/rust/performance.html",
        index::Context::Autotest,
        "performance",
    )
}
