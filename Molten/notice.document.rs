use vantle::Composition;

fn main() -> miette::Result<()> {
    let arguments = render::Arguments::parse();
    vantle::page(&arguments, "Notice", "molten", "notice", |c| {
        c.title("Notice")
            .rule()
            .paragraph(|p| p.text("Copyright 2025 Vantle"))
    })
}
