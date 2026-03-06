use miette::Diagnostic;
use thiserror::Error;

#[derive(Error, Debug, Diagnostic)]
pub enum Error {
    #[error("failed to parse source")]
    #[diagnostic(
        code(extract::parse),
        help("ensure source is valid for the given language")
    )]
    Parse,

    #[error("invalid tree-sitter query")]
    #[diagnostic(code(extract::invalid), help("check query syntax: {detail}"))]
    Invalid { detail: String },

    #[error("query matched no nodes")]
    #[diagnostic(code(extract::empty), help("no @capture match found{suggestion}"))]
    Empty { suggestion: String },

    #[error("query has no @capture pattern")]
    #[diagnostic(
        code(extract::capture),
        help("add a @capture name to your query pattern")
    )]
    Capture,

    #[error("unsupported language for extraction: {language}")]
    #[diagnostic(
        code(extract::unsupported),
        help("supported languages: rust, starlark, bash, json")
    )]
    Unsupported { language: String },

    #[error("failed to configure tree-sitter grammar")]
    #[diagnostic(code(extract::grammar), help("internal error: {detail}"))]
    Grammar { detail: String },

    #[error("failed to read source file: {path}")]
    #[diagnostic(code(extract::read), help("check file path and permissions"))]
    Read {
        path: String,
        #[source]
        source: std::io::Error,
    },

    #[error("failed to write output file: {path}")]
    #[diagnostic(
        code(extract::output),
        help("check write permissions and directory existence")
    )]
    Output {
        path: String,
        #[source]
        source: std::io::Error,
    },
}

impl Error {
    #[must_use]
    pub fn empty(names: &[String]) -> Self {
        let suggestion = similarity::nearest("capture", names).unwrap_or_default();
        Self::Empty { suggestion }
    }
}
