use body::Chain;
use navigation::Composition;

pub fn page(root: &str) -> page::Result {
    navigation::layout("Notice", &index::notice(root), root, |c| {
        c.title("Notice").rule().markdown("NOTICE.md")
    })
}
