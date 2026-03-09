#[must_use]
pub fn page(root: &str) -> page::Page {
    navigation::layout("Info", &index::info(root), root, |c| {
        c.title("Vantle")
            .rule()
            .paragraph(|p| {
                p.bold("Website: ")
                    .link("https://projects.vantle.org", |l| {
                        l.text("projects.vantle.org")
                    })
            })
            .paragraph(|p| {
                p.bold("Contact: ").link("mailto:connect@vantle.org", |l| {
                    l.text("connect@vantle.org")
                })
            })
            .paragraph(|p| p.text("Copyright \u{00a9} 2025-2026"))
    })
}
