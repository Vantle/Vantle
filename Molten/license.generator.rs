use vantle::Composition;

fn main() -> miette::Result<()> {
    let arguments = render::Arguments::parse();
    vantle::page(&arguments, "Molten License", |c| {
        c.title("License")
            .navigation(|n| {
                n.link("Readme.html", "Molten")
                    .link("../Readme.html", "Vantle")
                    .link("../Module.html", "Module")
            })
            .rule()
            .compose(vantle::license)
    })
}
