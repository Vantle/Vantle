fn main() -> miette::Result<()> {
    html::execute(|arguments| html::generate(arguments, notice::page(&arguments.root)))
}
