use miette::Diagnostic;
use thiserror::Error;

#[derive(Error, Debug, Diagnostic)]
pub enum Error {
    #[error("failed to highlight '{language}' code")]
    #[diagnostic(
        code(web::highlight),
        help("ensure a constructor exists for this language")
    )]
    Highlight { language: String },

    #[error("failed to render HTML")]
    #[diagnostic(code(web::render), help("check page structure for invalid elements"))]
    Render,

    #[error("failed to write output file: {path}")]
    #[diagnostic(
        code(web::output),
        help("check write permissions and directory existence")
    )]
    Output {
        path: String,
        #[source]
        source: std::io::Error,
    },
}

pub type Result<T> = miette::Result<T, Error>;
