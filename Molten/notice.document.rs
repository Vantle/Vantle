use body::Chain;
use style::Composition;

fn main() -> miette::Result<()> {
    html::execute(|arguments| {
        style::page(arguments, "Notice", "molten", "notice", |c| {
            c.title("Notice").rule().markdown("NOTICE.md")
        })
    })
}
