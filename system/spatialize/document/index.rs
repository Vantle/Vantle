#[must_use]
pub fn spatialize(root: &str) -> index::Index {
    index::Index::new(
        root,
        "system/spatialize/",
        index::Context::Vantle,
        "spatialize",
    )
}
