use miette::Diagnostic;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Diagnostic, Debug)]
pub enum Error {
    #[error("failed to activate concurrent runtime")]
    #[diagnostic(
        code(concurrent::activate),
        help("check system resources and thread limits")
    )]
    Activate {
        #[source]
        source: std::io::Error,
    },

    #[error("task failed to complete")]
    #[diagnostic(
        code(concurrent::join),
        help("the submitted task panicked or was cancelled")
    )]
    Join {
        #[source]
        source: tokio::task::JoinError,
    },
}
