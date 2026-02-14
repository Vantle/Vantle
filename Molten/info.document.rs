use vantle::Composition;

fn main() -> miette::Result<()> {
    let arguments = render::Arguments::parse();
    vantle::page(&arguments, "Info", "molten", "info", |c| {
        c.title("Vantle.Molten")
            .rule()
            .paragraph(|p| {
                p.bold("Website: ").link(
                    "https://projects.vantle.org/Molten",
                    "projects.vantle.org/Molten",
                )
            })
            .paragraph(|p| {
                p.bold("Contact: ").link(
                    "mailto:connect.molten@vantle.org",
                    "connect.molten@vantle.org",
                )
            })
            .paragraph(|p| p.text("Copyright \u{00a9} 2025"))
    })
}
