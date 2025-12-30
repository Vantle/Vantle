use miette::Diagnostic;
use thiserror::Error;

pub type Result<T> = miette::Result<T, Error>;

#[derive(Error, Debug, Diagnostic)]
pub enum Error {
    #[error("Missing {kind}: {label}")]
    #[diagnostic(code(inferencing::missing))]
    Missing {
        label: String,
        kind: String,
        #[help]
        suggestion: String,
    },
    #[error("Internal bug: duplicate destination {label}")]
    #[diagnostic(code(inferencing::duplicate))]
    Duplicate {
        label: String,
        #[help]
        suggestion: String,
    },
}

impl Error {
    #[must_use]
    pub fn node<L>(label: L) -> Self
    where
        L: Into<String>,
    {
        Error::Missing {
            label: label.into(),
            kind: "Node".to_string(),
            suggestion: "Ensure the node exists in the graph before querying".to_string(),
        }
    }

    #[must_use]
    pub fn edge<L>(label: L) -> Self
    where
        L: Into<String>,
    {
        Error::Missing {
            label: label.into(),
            kind: "Edge".to_string(),
            suggestion: "Ensure the edge exists in the graph before querying".to_string(),
        }
    }

    #[must_use]
    pub fn world<L>(label: L) -> Self
    where
        L: Into<String>,
    {
        Error::Missing {
            label: label.into(),
            kind: "World".to_string(),
            suggestion: "Ensure the node has been initialized".to_string(),
        }
    }

    #[must_use]
    pub fn duplicate<L>(label: L) -> Self
    where
        L: Into<String>,
    {
        Error::Duplicate {
            label: label.into(),
            suggestion: "This should never happen. Please report this bug".to_string(),
        }
    }

    #[must_use]
    pub fn refraction<L>(label: L) -> Self
    where
        L: Into<String>,
    {
        Error::Missing {
            label: label.into(),
            kind: "Refraction".to_string(),
            suggestion: "Ensure the refraction mapping exists for this label".to_string(),
        }
    }
}
