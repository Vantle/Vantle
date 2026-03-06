use body::Chain;
use extraction::Query;
use style::Composition;

fn main() -> miette::Result<()> {
    html::execute(|arguments| {
        style::page(arguments, "Module", "vantle", "module", |c| {
            c.title("Module")
                .element("a", |a| {
                    a.class("subtitle")
                        .attribute("href", "./MODULE.bazel")
                        .text("Bazel module definition")
                })
                .extract(module_source::EXTRACTIONS.one())
        })
    })
}
