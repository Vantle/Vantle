fn main() -> miette::Result<()> {
    html::execute(|arguments| html::generate(arguments, generation::page(&arguments.root)))
}
