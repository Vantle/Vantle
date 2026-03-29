use miette::Diagnostic;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Diagnostic, Debug)]
pub enum Error {
    #[error("unbalanced highlight element stack")]
    #[diagnostic(
        code(highlight::stack),
        help("node/end calls must be paired during syntax tree traversal")
    )]
    Stack,
}
