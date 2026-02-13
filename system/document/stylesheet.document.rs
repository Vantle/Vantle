fn main() -> miette::Result<()> {
    let arguments = render::Arguments::parse();
    render::stylesheet(&arguments, &vantle::theme())
}
