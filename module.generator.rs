use element::Language;
use vantle::Composition;

fn main() -> miette::Result<()> {
    let arguments = render::Arguments::parse();
    vantle::page(&arguments, "Module", |c| {
        c.title("Module")
            .subtitle("Bazel module definition")
            .navigation(|n| {
                n.link("Readme.html", "Vantle")
                    .link("License.html", "License")
            })
            .rule()
            .paragraph(|p| p.link("MODULE.bazel", "[source]"))
            .code("MODULE.bazel", Language::Starlark)
    })
}
