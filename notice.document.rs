use style::Composition;

fn main() -> miette::Result<()> {
    let arguments = html::Arguments::parse();
    style::page(&arguments, "Notice", "vantle", "notice", |c| {
        c.title("Notice")
            .rule()
            .paragraph(|p| p.text("Copyright 2025 Vantle"))
    })
}
