use element::Language;
use vantle::Composition;

fn main() -> miette::Result<()> {
    let arguments = render::Arguments::parse();
    vantle::page(&arguments, "Module", "vantle", "module", |c| {
        c.title("Module")
            .element("a", |a| {
                a.class("subtitle")
                    .attribute("href", "./MODULE.bazel")
                    .text("Bazel module definition")
            })
            .code("MODULE.bazel", Language::Starlark)
    })
}
