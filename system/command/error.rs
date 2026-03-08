#[derive(thiserror::Error, miette::Diagnostic, Debug)]
pub enum Error {
    #[error("{message}")]
    #[diagnostic(code(command::argument))]
    Argument {
        message: String,
        #[help]
        help: String,
    },

    #[error("flag {flag} requires a value")]
    #[diagnostic(code(command::flag), help("provide a value after {flag}"))]
    Flag { flag: String },

    #[error("failed to write query output to {path}")]
    #[diagnostic(code(command::output), help("check that the output path is writable"))]
    Output {
        path: String,
        #[source]
        source: std::io::Error,
    },
}

pub type Result<T> = std::result::Result<T, Error>;
