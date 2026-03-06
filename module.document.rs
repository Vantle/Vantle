fn main() -> miette::Result<()> {
    html::execute(|arguments| html::generate(arguments, module::page(&arguments.root)))
}
