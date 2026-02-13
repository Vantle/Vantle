use element::Language;
use vantle::Composition;

fn main() -> miette::Result<()> {
    let arguments = render::Arguments::parse();
    vantle::page(&arguments, "Module", "vantle", "module", |c| {
        c.title("Module")
            .subtitle("Bazel module definition")
            .rule()
            .paragraph(|p| p.link("MODULE.bazel", "[source]"))
            .code("MODULE.bazel", Language::Starlark)
    })
}
