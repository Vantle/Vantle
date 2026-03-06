fn main() -> miette::Result<()> {
    html::execute(|arguments| html::generate(arguments, vantle::page(&arguments.root)))
}
