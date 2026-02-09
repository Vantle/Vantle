use vantle::Composition;

fn main() -> miette::Result<()> {
    let arguments = render::Arguments::parse();
    vantle::page(&arguments, "Notice", |c| {
        c.title("Notice")
            .navigation(|n| {
                n.link("Readme.html", "Vantle")
                    .link("Module.html", "Module")
                    .link("License.html", "License")
            })
            .rule()
            .paragraph(|p| p.text("Copyright 2025 Vantle"))
    })
}
