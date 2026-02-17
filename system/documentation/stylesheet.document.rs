fn main() -> miette::Result<()> {
    let arguments = html::Arguments::parse();
    html::stylesheet(&arguments, &style::theme())
}
