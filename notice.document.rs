use style::Composition;

fn main() -> miette::Result<()> {
    let arguments = html::Arguments::parse();
    style::page(&arguments, "Notice", "vantle", "notice", |c| {
        c.title("Notice").rule().markdown("NOTICE.md")
    })
}
