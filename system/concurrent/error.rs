use miette::Diagnostic;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Diagnostic, Debug)]
pub enum Error {
    #[error("failed to create concurrent runtime")]
    #[diagnostic(code(runtime::create), help("check system resource limits"))]
    Runtime {
        #[source]
        source: std::io::Error,
    },

    #[error("task was cancelled")]
    #[diagnostic(code(runtime::cancelled), help("shutdown was initiated"))]
    Cancelled,

    #[error("failed to spawn concurrent task")]
    #[diagnostic(code(runtime::spawn), help("runtime may be shutting down"))]
    Spawn,
}
