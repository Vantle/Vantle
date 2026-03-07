fn main() -> miette::Result<()> {
    html::execute(|arguments| html::generate(arguments, performance::page(&arguments.root)))
}
