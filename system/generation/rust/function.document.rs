fn main() -> miette::Result<()> {
    html::execute(|arguments| html::generate(arguments, function::page(&arguments.root)))
}
