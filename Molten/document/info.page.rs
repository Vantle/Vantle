#[must_use]
pub fn page(root: &str) -> page::Page {
    navigation::layout("Info", &index::molten::info(root), root, |c| {
        c.title("Vantle.Molten")
            .rule()
            .paragraph(|p| {
                p.bold("Website: ")
                    .link("https://projects.vantle.org/Molten", |l| {
                        l.text("projects.vantle.org/Molten")
                    })
            })
            .paragraph(|p| {
                p.bold("Contact: ")
                    .link("mailto:connect.molten@vantle.org", |l| {
                        l.text("connect.molten@vantle.org")
                    })
            })
            .paragraph(|p| p.text("Copyright \u{00a9} 2025-2026"))
    })
}
