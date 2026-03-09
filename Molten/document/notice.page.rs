#[must_use]
pub fn page(root: &str) -> page::Page {
    content::page(
        "Notice",
        &index::molten::notice(root),
        root,
        content::notice(),
    )
}
