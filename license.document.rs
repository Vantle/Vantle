use style::Composition;

fn main() -> miette::Result<()> {
    html::execute(|arguments| {
        style::page(arguments, "License", "vantle", "license", |c| {
            c.title("License").rule().markdown("LICENSE.md")
        })
    })
}
