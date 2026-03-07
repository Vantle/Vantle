use miette::{Diagnostic, NamedSource, SourceSpan};
use serde_json::{Map, Value};
use thiserror::Error;

#[derive(Error, Debug, Diagnostic)]
pub enum Error {
    #[error("assertion mismatch")]
    #[diagnostic(code(report::mismatch))]
    Mismatch {
        actuals: Map<String, Value>,
        #[source_code]
        source_code: NamedSource<String>,
        #[label("mismatch here")]
        span: SourceSpan,
        #[help]
        help: String,
    },

    #[error("{count} test case{plural} failed")]
    #[diagnostic(code(report::collection), url("{output}"))]
    Collection {
        count: usize,
        plural: &'static str,
        output: String,
        #[related]
        failures: Vec<Failure>,
    },

    #[error("failed to serialize return value")]
    #[diagnostic(code(report::serialization))]
    Serialization {
        #[source_code]
        source: NamedSource<String>,
        #[label("this value could not be serialized")]
        span: SourceSpan,
        #[help]
        help: String,
        #[source]
        cause: serde_json::Error,
    },

    #[error("report write failed")]
    #[diagnostic(code(report::write))]
    Write {
        #[help]
        help: String,
        #[source]
        cause: std::io::Error,
    },

    #[error(transparent)]
    #[diagnostic(transparent)]
    Runtime(#[from] runtime::Runtime),
}

#[derive(Error, Debug, Diagnostic)]
#[error("{name}")]
#[diagnostic(code(report::failure))]
pub struct Failure {
    name: String,
    #[help]
    help: String,
}

impl From<runtime::Runtime> for Box<Error> {
    fn from(error: runtime::Runtime) -> Self {
        Box::new(Error::Runtime(error))
    }
}

impl Failure {
    #[must_use]
    pub fn new(name: String, help: String) -> Self {
        Self { name, help }
    }
}

impl Error {
    #[must_use]
    pub fn collection(failures: Vec<Failure>, output: &std::path::Path) -> Self {
        let count = failures.len();
        Self::Collection {
            count,
            plural: if count == 1 { "" } else { "s" },
            output: output.display().to_string(),
            failures,
        }
    }

    #[must_use]
    pub fn mismatch(actuals: Map<String, Value>, help: String) -> Self {
        let content = serde_json::to_string_pretty(&actuals).unwrap_or_default();
        let span = SourceSpan::new(0.into(), content.len());
        Self::Mismatch {
            actuals,
            source_code: NamedSource::new("actuals", content),
            span,
            help,
        }
    }

    #[must_use]
    pub fn serialization(key: &str, cause: serde_json::Error) -> Self {
        let content = format!("key: {key}");
        let span = SourceSpan::new(0.into(), content.len());
        Self::Serialization {
            source: NamedSource::new("serialization", content),
            span,
            help: format!("the value for '{key}' could not be serialized to JSON"),
            cause,
        }
    }

    #[must_use]
    pub fn write(path: &std::path::Path, cause: std::io::Error) -> Self {
        Self::Write {
            help: format!("failed to write report to '{}'", path.display()),
            cause,
        }
    }
}
