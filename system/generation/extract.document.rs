fn main() -> miette::Result<()> {
    html::execute(|arguments| html::generate(arguments, extract::page(&arguments.root)))
}
