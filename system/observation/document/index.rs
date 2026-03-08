#[must_use]
pub fn observation(root: &str) -> index::Index {
    index::Index::new(
        root,
        "system/observation/",
        index::Context::Vantle,
        "observation",
    )
}
