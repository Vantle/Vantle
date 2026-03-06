fn main() -> miette::Result<()> {
    html::execute(|arguments| html::generate(arguments, molten::page(&arguments.root)))
}
