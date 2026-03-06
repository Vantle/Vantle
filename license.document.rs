fn main() -> miette::Result<()> {
    html::execute(|arguments| html::generate(arguments, license::page(&arguments.root)))
}
