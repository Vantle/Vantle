fn main() -> miette::Result<()> {
    html::execute(|arguments| html::generate(arguments, observation::page(&arguments.root)))
}
