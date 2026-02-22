use style::Composition;

fn main() -> miette::Result<()> {
    let arguments = html::Arguments::parse();
    style::page(&arguments, "License", "vantle", "license", |c| {
        c.title("License").rule().markdown("LICENSE.md")
    })
}
