fn main() -> miette::Result<()> {
    html::execute(|arguments| html::generate(arguments, info::page(&arguments.root)))
}
