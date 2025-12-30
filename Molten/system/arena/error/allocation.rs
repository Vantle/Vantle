use miette::Diagnostic;
use thiserror::Error;

#[derive(Error, Debug, Diagnostic)]
pub enum Allocation {
    #[error("Arena allocation limit reached")]
    #[diagnostic(
        code(arena::allocation::limit),
        help(
            "The arena has reached its maximum capacity. Consider optimizing your data structure or increasing limits."
        )
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
    #[must_use]
    pub fn collision(index: usize) -> Self {
        Self::Collision { index }
    }

    #[must_use]
    pub fn capacity(current_size: usize, max_size: usize) -> Self {
        let message = format!(
            "{current_size} elements, maximum allowed: {max_size}. Consider reducing elements or optimizing your data structure."
        );

        Self::Capacity { message }
    }

    #[must_use]
    pub fn unification<E>(element: E) -> Self
    where
        E: std::fmt::Debug,
    {
        Self::Unification {
            element: format!("{element:#?}"),
        }
    }
}
