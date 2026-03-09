#[derive(thiserror::Error, miette::Diagnostic, Debug)]
pub enum Error {
    #[error("failed to create element '{name}'")]
    #[diagnostic(code(assembly::element), help("ensure the document is valid"))]
    Element { name: String },

    #[error("failed to set attribute '{key}' on element")]
    #[diagnostic(code(assembly::attribute), help("ensure the attribute name is valid"))]
    Attribute { key: String },

    #[error("failed to append child to parent")]
    #[diagnostic(code(assembly::append), help("ensure the DOM tree is valid"))]
    Append,

    #[error("emitter stack underflow")]
    #[diagnostic(
        code(assembly::stack),
        help("ensure open and close calls are balanced")
    )]
    Stack,
}
