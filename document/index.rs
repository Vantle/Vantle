pub use generation;
pub use molten;
pub use observation;
pub use spatialize;

#[must_use]
pub fn readme(root: &str) -> index::Index {
    index::Index::new(root, "index.html", index::Context::Vantle, "readme")
}

#[must_use]
pub fn info(root: &str) -> index::Index {
    index::Index::new(root, "info.html", index::Context::Vantle, "info")
}

#[must_use]
pub fn notice(root: &str) -> index::Index {
    index::Index::new(root, "notice.html", index::Context::Vantle, "notice")
}

#[must_use]
pub fn license(root: &str) -> index::Index {
    index::Index::new(root, "license.html", index::Context::Vantle, "license")
}

#[must_use]
pub fn module(root: &str) -> index::Index {
    index::Index::new(root, "module.html", index::Context::Vantle, "module")
}
