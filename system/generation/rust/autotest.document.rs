fn main() -> miette::Result<()> {
    html::execute(|arguments| html::generate(arguments, autotest::page(&arguments.root)))
}
