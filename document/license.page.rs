#[must_use]
pub fn page(root: &str) -> page::Page {
    content::page("License", &index::license(root), root, content::license())
}
