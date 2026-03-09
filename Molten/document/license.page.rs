#[must_use]
pub fn page(root: &str) -> page::Page {
    content::page(
        "License",
        &index::molten::license(root),
        root,
        content::license(),
    )
}
