use miette::Diagnostic;
use thiserror::Error as ThisError;

pub type Result<T> = miette::Result<T, Error>;

#[derive(ThisError, Debug, Diagnostic)]
pub enum Error {
    #[error(transparent)]
    #[diagnostic(code(translator::io))]
    Io(#[from] std::io::Error),
}
