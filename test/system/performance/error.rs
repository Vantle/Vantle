use miette::Diagnostic;
use thiserror::Error;

#[derive(Error, Debug, Diagnostic)]
pub enum Error {
    #[error("performance bound exceeded")]
    #[diagnostic(code(performance::regression), help("{help}"))]
    Regression { help: String },

    #[error("correctness assertion failed during performance measurement")]
    #[diagnostic(code(performance::correctness), help("{help}"))]
    Correctness { help: String },

    #[error("insufficient data for model inference")]
    #[diagnostic(code(performance::insufficient), help("{help}"))]
    Insufficient { help: String },

    #[error("report write failed")]
    #[diagnostic(code(performance::write))]
    Write {
        #[help]
        help: String,
        #[source]
        cause: std::io::Error,
    },

    #[error("{count} performance bound{plural} violated")]
    #[diagnostic(code(performance::collection))]
    Collection {
        count: usize,
        plural: &'static str,
        #[related]
        failures: Vec<Violation>,
    },
}

#[derive(Error, Debug, Diagnostic)]
#[error("{name}")]
#[diagnostic(code(performance::violation))]
pub struct Violation {
    name: String,
    #[help]
    help: String,
}

impl Violation {
    #[must_use]
    pub fn new(name: String, help: String) -> Self {
        Self { name, help }
    }
}

impl Error {
    #[must_use]
    pub fn collection(failures: Vec<Violation>) -> Self {
        let count = failures.len();
        Self::Collection {
            count,
            plural: if count == 1 { "" } else { "s" },
            failures,
        }
    }

    #[must_use]
    pub fn write(path: &std::path::Path, cause: std::io::Error) -> Self {
        Self::Write {
            help: format!("failed to write performance report to '{}'", path.display()),
            cause,
        }
    }
}
