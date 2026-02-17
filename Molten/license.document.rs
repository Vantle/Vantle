use style::Composition;

fn main() -> miette::Result<()> {
    let arguments = html::Arguments::parse();
    style::page(&arguments, "License", "molten", "license", |c| {
        c.title("License").rule().compose(style::license)
    })
}
