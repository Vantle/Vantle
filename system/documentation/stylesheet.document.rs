fn main() -> miette::Result<()> {
    html::execute(|arguments| html::stylesheet(arguments, &style::theme()))
}
