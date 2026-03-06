use body::Chain;
use style::Composition;

pub fn page(root: &str) -> page::Result {
    style::layout("Notice", "molten", "notice", root, |c| {
        c.title("Notice").rule().markdown("NOTICE.md")
    })
}
