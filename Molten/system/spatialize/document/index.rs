#[must_use]
pub fn spatialize(root: &str) -> index::Index {
    index::Index::new(
        root,
        "Molten/system/spatialize/",
        index::Context::Molten,
        "spatialize",
    )
}
