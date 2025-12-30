#[derive(thiserror::Error, miette::Diagnostic, Debug)]
pub enum Error {
    #[error("failed to read syntax file: {path}")]
    #[diagnostic(
        code(graph::symbolic::highlighter::read),
        help("ensure Molten/resource/system/graph/syntax.yaml exists")
    )]
    Read {
        path: String,
        #[source]
        source: std::io::Error,
    },

    #[error("failed to parse syntax definition: {path}: {details}")]
    #[diagnostic(
        code(graph::symbolic::highlighter::parse),
        help("check syntax.yaml for valid sublimetext syntax format")
    )]
    Parse { path: String, details: String },
}

pub type Result<T> = std::result::Result<T, Error>;
