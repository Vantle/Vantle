//! Allocation errors for the arena component.

use miette::Diagnostic;
use thiserror::Error;

#[derive(Error, Debug, Diagnostic)]
pub enum Allocation {
    #[error("Arena allocation limit reached")]
    #[diagnostic(
        code(arena::allocation::limit),
        help("The arena has reached its maximum capacity. Consider optimizing your data structure or increasing limits.")
    )]
    Limit,

    #[error("Index collision in arena at position {index}")]
    #[diagnostic(
        code(arena::allocation::collision),
        help(
            "This indicates a bug in the arena implementation. The same index was allocated twice."
        )
    )]
    Collision { index: usize },

    #[error("Arena capacity exceeded: {message}")]
    #[diagnostic(
        code(arena::allocation::capacity),
        help("Try reducing the number of elements or optimizing your data structure")
    )]
    Capacity { message: String },

    #[error("Constraint unification failed for element: {element}")]
    #[diagnostic(
        code(arena::allocation::unification),
        help("The element could not be unified with existing constraints in the arena")
    )]
    Unification { element: String },
}

impl Allocation {
    pub fn code(&self) -> i32 {
        match self {
            Self::Limit => 65,
            Self::Collision { .. } => 66,
            Self::Capacity { .. } => 68,
            Self::Unification { .. } => 67,
        }
    }

    pub fn collision(index: usize) -> Self {
        Self::Collision { index }
    }

    pub fn capacity(current_size: usize, max_size: usize) -> Self {
        let message = format!(
            "{} elements, maximum allowed: {}. Consider reducing elements or optimizing your data structure.",
            current_size, max_size
        );

        Self::Capacity { message }
    }

    pub fn unification(element: impl std::fmt::Debug) -> Self {
        Self::Unification {
            element: format!("{:#?}", element),
        }
    }
}
