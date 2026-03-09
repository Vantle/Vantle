use miette::Diagnostic;
use thiserror::Error;

#[derive(Error, Diagnostic, Debug)]
pub enum Error {
    #[error("failed to parse {language}")]
    #[diagnostic(
        code(constructor::parse),
        help("check that the source is valid {language}")
    )]
    Parse { language: &'static str },

    #[error("failed to configure {language} parser")]
    #[diagnostic(
        code(constructor::language),
        help("tree-sitter grammar may be incompatible")
    )]
    Language {
        language: &'static str,
        #[source]
        source: tree_sitter::LanguageError,
    },

    #[error("failed to parse rust")]
    #[diagnostic(code(constructor::rust), help("check that the source is valid rust"))]
    Rust {
        #[source]
        source: syn::Error,
    },

    #[error("failed to parse json")]
    #[diagnostic(code(constructor::json), help("check that the source is valid json"))]
    Json {
        #[source]
        source: serde_json::Error,
    },
}
