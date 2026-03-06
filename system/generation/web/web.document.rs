fn main() -> miette::Result<()> {
    html::execute(|arguments| html::generate(arguments, web::page(&arguments.root)))
}
