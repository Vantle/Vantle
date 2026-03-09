#[must_use]
pub fn license() -> Vec<element::Element> {
    markdown::parse(include_str!("../LICENSE.md"))
}

#[must_use]
pub fn notice() -> Vec<element::Element> {
    markdown::parse(include_str!("../NOTICE.md"))
}

#[must_use]
pub fn page(
    title: &str,
    index: &entry::Index,
    root: &str,
    elements: Vec<element::Element>,
) -> page::Page {
    navigation::layout(title, index, root, |c| {
        c.title(title).rule().markdown(elements)
    })
}
