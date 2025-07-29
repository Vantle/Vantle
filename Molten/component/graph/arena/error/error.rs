//! Error types for the arena component.

use miette::Diagnostic;
use thiserror::Error;

// Re-export the allocation error module directly
pub use allocation;

/// Missing element error
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
    /// Get the error code for this error type
    pub fn code(&self) -> i32 {
        match self {
            Self::Element { .. } => 64,
        }
    }

    /// Create a beautiful missing element error
    pub fn element(element: impl std::fmt::Debug) -> Self {
        Self::Element {
            element: format!("{:#?}", element),
        }
    }
}

/// Arena error that combines missing and allocation errors for trait compatibility
#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Missing(#[from] Missing),

    #[error(transparent)]
    Allocation(#[from] allocation::Allocation),
}
