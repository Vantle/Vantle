use element::Language;
use style::Composition;

fn main() -> miette::Result<()> {
    let arguments = html::Arguments::parse();
    style::page(&arguments, "Module", "vantle", "module", |c| {
        c.title("Module")
            .element("a", |a| {
                a.class("subtitle")
                    .attribute("href", "./MODULE.bazel")
                    .text("Bazel module definition")
            })
            .code("MODULE.bazel", Language::Starlark)
    })
}
