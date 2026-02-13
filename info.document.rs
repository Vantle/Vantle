use vantle::Composition;

fn main() -> miette::Result<()> {
    let arguments = render::Arguments::parse();
    vantle::page(&arguments, "Info", "vantle", "info", |c| {
        c.title("Vantle")
            .rule()
            .paragraph(|p| {
                p.bold("Website: ")
                    .link("https://projects.vantle.org", "projects.vantle.org")
            })
            .paragraph(|p| {
                p.bold("Contact: ")
                    .link("mailto:connect@vantle.org", "connect@vantle.org")
            })
            .paragraph(|p| p.text("Copyright \u{00a9} 2025"))
    })
}
