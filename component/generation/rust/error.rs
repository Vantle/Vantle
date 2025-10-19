//! Error types for the generator component.

use miette::{Diagnostic, NamedSource, SourceSpan};
use thiserror::Error;

#[derive(Error, Debug, Diagnostic)]
pub enum Error {
    #[error("IO error: {0}")]
    #[diagnostic(code(generator::io_error))]
    Io(#[from] std::io::Error),

    #[error("JSON parsing error: {0}")]
    #[diagnostic(code(generator::json_error))]
    Json(#[from] serde_json::Error),

    #[error("Syntax error: {0}")]
    #[diagnostic(code(generator::syntax_error))]
    Syntax(#[from] syn::Error),

    #[error("Unsupported language: {0}")]
    #[diagnostic(code(generator::unsupported_language))]
    Language(String),

    #[error("Missing field '{field}' in {context}")]
    #[diagnostic(
        code(generator::missing_field),
        help("Check that all required fields are present in your configuration")
    )]
    Missing { field: String, context: String },

    #[error("Function '{name}' not found in template")]
    #[diagnostic(
        code(generator::function_not_found),
        help("Available functions are listed in the error message above")
    )]
    NotFound { name: String },

    #[error("Template validation failed")]
    #[diagnostic(code(generator::template_validation_failed))]
    Template {
        #[source_code]
        code: NamedSource<String>,
        #[label("Function declared here")]
        span: Option<SourceSpan>,
        #[help]
        help: String,
    },

    #[error("Function not found in template")]
    #[diagnostic(code(generator::cases_validation_failed))]
    Cases {
        #[source_code]
        code: NamedSource<String>,
        #[label("This function doesn't exist in the template")]
        span: Option<SourceSpan>,
        #[help]
        help: String,
    },

    #[error("Parameter validation failed")]
    #[diagnostic(code(generator::parameter_validation_failed))]
    Parameter {
        #[source_code]
        code: NamedSource<String>,
        #[label("Missing or invalid parameter")]
        span: Option<SourceSpan>,
        #[help]
        help: String,
    },

    #[error("Test case validation failed")]
    #[diagnostic(code(generator::test_case_validation_failed))]
    Case {
        #[source_code]
        code: NamedSource<String>,
        #[label("Problem in this test case")]
        span: Option<SourceSpan>,
        #[help]
        help: String,
    },

    #[error("Type validation failed")]
    #[diagnostic(code(generator::type_validation_failed))]
    Type {
        #[source_code]
        code: NamedSource<String>,
        #[label("Type mismatch")]
        span: Option<SourceSpan>,
        #[help]
        help: String,
    },

    #[error("Deserialization")]
    #[diagnostic(code(generator::deserialization))]
    Deserialization {
        #[source_code]
        json: NamedSource<String>,

        #[label("Invalid syntax here")]
        location: SourceSpan,

        #[help]
        help: String,

        #[source]
        cause: serde_json::Error,
    },
}

impl Error {
    pub fn code(&self) -> i32 {
        match self {
            Self::Language(_) => 64,
            Self::Json(_) | Self::Syntax(_) | Self::Missing { .. } => 65,
            Self::Io(_) => 66,
            Self::NotFound { .. } => 65,
            Self::Template { .. } => 65,
            Self::Cases { .. } => 65,
            Self::Parameter { .. } => 65,
            Self::Case { .. } => 65,
            Self::Type { .. } => 65,
            Self::Deserialization { .. } => 65,
        }
    }

    pub fn template(
        path: &str,
        content: String,
        function: SourceSpan,
        available: &[String],
    ) -> Self {
        Self::Template {
            code: NamedSource::new(path, content).with_language(language::rust()),
            span: Some(function),
            help: format!("Available: [{}]", available.join(", ")),
        }
    }

    pub fn cases(path: &str, content: String, span: Option<(usize, usize)>, help: String) -> Self {
        Self::Cases {
            code: NamedSource::new(path, content).with_language(language::json()),
            span: span.map(|(start, len)| SourceSpan::new(start.into(), len)),
            help,
        }
    }

    pub fn missing(
        path: &str,
        content: String,
        problem: Option<(usize, usize)>,
        missing: &str,
        function: &str,
        available: &[String],
    ) -> Self {
        let help = format!(
            "Parameter '{}' is required for function '{}'.\n\nExpected parameters for this function:\n{}\n\nTip: Add the missing parameter '{}' to the \"parameters\" object in your test case.",
            missing,
            function,
            available.iter()
                .map(|p| format!("  • {}", p))
                .collect::<Vec<_>>()
                .join("\n"),
            missing
        );

        Self::Parameter {
            code: NamedSource::new(path, content).with_language(language::json()),
            span: problem.map(|(start, len)| SourceSpan::new(start.into(), len)),
            help,
        }
    }

    pub fn extraneous(
        path: &str,
        content: String,
        problem: Option<(usize, usize)>,
        extra: &str,
        function: &str,
        expected: &[String],
    ) -> Self {
        let help = format!(
            "Parameter '{}' is not expected by function '{}'.\n\nExpected parameters for this function:\n{}\n\nTip: Remove the extra parameter '{}' from your test case or check the function signature.",
            extra,
            function,
            expected.iter()
                .map(|p| format!("  • {}", p))
                .collect::<Vec<_>>()
                .join("\n"),
            extra
        );

        Self::Parameter {
            code: NamedSource::new(path, content).with_language(language::json()),
            span: problem.map(|(start, len)| SourceSpan::new(start.into(), len)),
            help,
        }
    }

    pub fn test(
        case: &str,
        content: String,
        span: Option<(usize, usize)>,
        issue: &str,
        suggestion: &str,
    ) -> Self {
        let help = format!("Test case issue: {}\n\nSuggestion: {}", issue, suggestion);

        Self::Case {
            code: NamedSource::new(case, content).with_language(language::json()),
            span: span.map(|(start, len)| SourceSpan::new(start.into(), len)),
            help,
        }
    }

    pub fn typing(
        path: &str,
        content: String,
        span: Option<(usize, usize)>,
        parameter: &str,
        expected: &str,
        actual: &str,
        context: &str,
    ) -> Self {
        let help = format!(
            "Parameter '{}' expects type '{}' but got {}.\n\nContext: {}\n\nTip: Check that the value matches the expected Rust type.",
            parameter,
            expected,
            actual,
            context
        );

        Self::Type {
            code: NamedSource::new(path, content).with_language(language::json()),
            span: span.map(|(start, len)| SourceSpan::new(start.into(), len)),
            help,
        }
    }

    pub fn deserialization(
        target: &str,
        source: impl AsRef<std::path::Path>,
        json: impl Into<String>,
        error: serde_json::Error,
    ) -> Self {
        let json = json.into();
        let source_path = source.as_ref();

        let offset = {
            let line = error.line().saturating_sub(1);
            let column = error.column().saturating_sub(1);

            let mut offset = 0;
            for (index, row) in json.lines().enumerate() {
                if index == line {
                    offset += column;
                    break;
                }
                offset += row.len() + 1;
            }
            offset
        };

        let help = format!(
            "In file: {}\nExpected type: {}\nCheck that your structure matches the expected target format.",
            source_path.display(),
            target
        );

        Self::Deserialization {
            json: NamedSource::new(source_path.to_string_lossy(), json)
                .with_language(language::json()),
            location: SourceSpan::new(offset.into(), 1),
            help,
            cause: error,
        }
    }
}
pub type Result<T> = std::result::Result<T, Error>;
