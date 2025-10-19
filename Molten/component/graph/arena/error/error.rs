use miette::Diagnostic;
use thiserror::Error;

pub use allocation;

#[derive(Error, Debug, Diagnostic)]
pub enum Missing {
    #[error("Element not found in arena: {element}")]
    #[diagnostic(
        code(arena::missing::element),
        help("Check that the element was properly added to the arena before trying to access it")
    )]
    Element { element: String },
}

impl Missing {
    pub fn element(element: impl std::fmt::Debug) -> Self {
        Self::Element {
            element: format!("{:#?}", element),
        }
    }
}

#[derive(Error, Debug, Diagnostic)]
pub enum Error {
    #[error(transparent)]
    #[diagnostic(transparent)]
    Missing(#[from] Missing),

    #[error(transparent)]
    #[diagnostic(transparent)]
    Allocation(#[from] allocation::Allocation),
}
