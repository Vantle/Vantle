//! Error types for the generator component.

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("JSON parsing error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("Syntax error: {0}")]
    Syntax(#[from] syn::Error),
    #[error("Unsupported language: {0}")]
    Language(String),
    #[error("Missing field '{field}' in {context}")]
    Missing { field: String, context: String },
    #[error("Function '{name}' not found")]
    NotFound { name: String },
}

impl Error {
    pub fn code(&self) -> i32 {
        match self {
            Self::Language(_) => 64,
            Self::Json(_) | Self::Syntax(_) | Self::Missing { .. } => 65,
            Self::Io(_) => 66,
            Self::NotFound { .. } => 65,
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;
