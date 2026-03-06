use miette::Diagnostic;
use thiserror::Error;

pub type Result<T> = miette::Result<T, Error>;

#[derive(Error, Debug, Diagnostic)]
pub enum Error {
    #[error(transparent)]
    #[diagnostic(code(translator::io), help("check file permissions and path validity"))]
    Io(#[from] std::io::Error),
}
