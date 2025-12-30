use miette::{Diagnostic, NamedSource, SourceSpan};
use thiserror::Error;

pub type Result<T> = miette::Result<T, Error>;

#[derive(Error, Debug, Diagnostic)]
pub enum Error {
    #[error("Element invalid in {context}")]
    #[diagnostic(code(constructor::unexpected))]
    Unexpected {
        token: String,
        expected: String,
        context: String,
        #[label("expected {expected}")]
        span: SourceSpan,
    },

    #[error("{context} yields undefined state")]
    #[diagnostic(code(constructor::undefined))]
    Undefined {
        #[label("Undefined state here")]
        span: SourceSpan,
        context: String,
    },

    #[error("Expected element `{token}` not defined `{context}`")]
    #[diagnostic(code(constructor::incomplete))]
    Incomplete {
        token: String,
        #[label("Incomplete here")]
        span: SourceSpan,
        context: String,
    },

    #[error(transparent)]
    #[diagnostic(transparent)]
    Translator(#[from] translator::error::Error),

    #[error(transparent)]
    Io(#[from] std::io::Error),
}

#[derive(Error, Debug, Diagnostic)]
#[error("Failed to parse named source input")]
#[diagnostic(code(constructor::parse))]
pub struct Sourced {
    #[diagnostic_source]
    pub error: Error,
    #[source_code]
    pub location: NamedSource<String>,
}

impl Sourced {
    #[must_use]
    pub fn wrap(source: NamedSource<String>, error: Error) -> Self {
        Sourced {
            error,
            location: source.with_language(language::molten()),
        }
    }
}
