pub use error::Error;

use cached::proc_macro::once;
use observe::trace;
use syntect::parsing::SyntaxDefinition;

#[once(result = true)]
#[trace(channels = [core])]
pub fn syntax() -> error::Result<SyntaxDefinition> {
    let path = "Molten/resource/system/graph/syntax.yaml";
    let content = std::fs::read_to_string(path).map_err(|source| Error::Read {
        path: path.to_string(),
        source,
    })?;
    SyntaxDefinition::load_from_str(&content, false, None).map_err(|source| Error::Parse {
        path: path.to_string(),
        details: source.to_string(),
    })
}

mod error {
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
}
