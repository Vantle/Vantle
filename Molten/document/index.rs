pub use spatialize;

#[must_use]
pub fn readme(root: &str) -> index::Index {
    index::Index::new(root, "Molten/", index::Context::Molten, "readme")
}

#[must_use]
pub fn info(root: &str) -> index::Index {
    index::Index::new(root, "Molten/info.html", index::Context::Molten, "info")
}

#[must_use]
pub fn notice(root: &str) -> index::Index {
    index::Index::new(root, "Molten/notice.html", index::Context::Molten, "notice")
}

#[must_use]
pub fn license(root: &str) -> index::Index {
    index::Index::new(
        root,
        "Molten/license.html",
        index::Context::Molten,
        "license",
    )
}
