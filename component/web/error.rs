use miette::Diagnostic;
use thiserror::Error;

#[derive(Error, Debug, Diagnostic)]
pub enum Error {
    #[error("source file '{name}' not found in data map")]
    #[diagnostic(
        code(web::source),
        help("available sources: [{available}]{suggestion}")
    )]
    Source {
        name: String,
        available: String,
        suggestion: String,
    },

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
    Output { path: String },

    #[error("generated files have drifted from repository: {files}")]
    #[diagnostic(
        code(web::drift),
        help("run 'bazel run //:generate.documentation' to update")
    )]
    Drift { files: String },
}

impl Error {
    #[must_use]
    pub fn source(name: &str, available: &[String]) -> Self {
        let suggestion = similarity::nearest(name, available).unwrap_or_default();
        Self::Source {
            name: name.into(),
            available: available.join(", "),
            suggestion,
        }
    }
}

pub type Result<T> = miette::Result<T, Error>;
