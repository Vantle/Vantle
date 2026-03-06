use miette::Diagnostic;
use thiserror::Error;

pub type Result<T> = miette::Result<T, Error>;

#[derive(Error, Debug, Diagnostic)]
pub enum Error {
    #[error(transparent)]
    #[diagnostic(transparent)]
    Translator(#[from] translate::Error),

    #[error(transparent)]
    #[diagnostic(code(expression::io), help("check input validity"))]
    Io(#[from] std::io::Error),
}
