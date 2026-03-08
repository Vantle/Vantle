use body::Chain;
use navigation::Composition;

pub fn page(root: &str) -> page::Result {
    navigation::layout("License", &index::license(root), root, |c| {
        c.title("License").rule().markdown("LICENSE.md")
    })
}
