use body::Chain;
use style::Composition;

fn main() -> miette::Result<()> {
    html::execute(|arguments| {
        style::page(arguments, "License", "molten", "license", |c| {
            c.title("License").rule().markdown("LICENSE.md")
        })
    })
}
