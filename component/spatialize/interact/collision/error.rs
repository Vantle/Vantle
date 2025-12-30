use miette::Diagnostic;
use thiserror::Error;

#[derive(Error, Diagnostic, Debug)]
pub enum Error {
    #[error("invalid envelope: minimum exceeds maximum")]
    #[diagnostic(
        code(collision::envelope),
        help("Ensure minimum coordinates are less than maximum")
    )]
    Envelope,

    #[error("invalid radius: must be positive")]
    #[diagnostic(code(collision::radius), help("Provide a positive radius value"))]
    Radius,
}
