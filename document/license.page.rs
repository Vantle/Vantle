use body::Chain;
use style::Composition;

pub fn page(root: &str) -> page::Result {
    style::layout("License", "vantle", "license", root, |c| {
        c.title("License").rule().markdown("LICENSE.md")
    })
}
