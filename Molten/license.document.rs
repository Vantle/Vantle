use vantle::Composition;

fn main() -> miette::Result<()> {
    let arguments = render::Arguments::parse();
    vantle::page(&arguments, "License", "molten", "license", |c| {
        c.title("License").rule().compose(vantle::license)
    })
}
