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
    Untargetable { name: String },

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
    #[must_use]
    pub fn code(&self) -> i32 {
        match self {
            Self::Language(_) => 64,
            Self::Io(_) => 66,
            Self::Json(_)
            | Self::Syntax(_)
            | Self::Missing { .. }
            | Self::Untargetable { .. }
            | Self::Template { .. }
            | Self::Cases { .. }
            | Self::Parameter { .. }
            | Self::Case { .. }
            | Self::Type { .. }
            | Self::Deserialization { .. } => 65,
        }
    }

    #[must_use]
    pub fn template(
        path: &str,
        content: String,
        function: SourceSpan,
        name: &str,
        available: &[String],
    ) -> Self {
        let suggestion = similarity::nearest(name, available).unwrap_or_default();

        Self::Template {
            code: NamedSource::new(path, content).with_language(language::rust()),
            span: Some(function),
            help: format!("Available: [{}]{}", available.join(", "), suggestion),
        }
    }

    #[must_use]
    pub fn cases(path: &str, content: String, span: Option<(usize, usize)>, help: String) -> Self {
        Self::Cases {
            code: NamedSource::new(path, content).with_language(language::json()),
            span: span.map(|(start, len)| SourceSpan::new(start.into(), len)),
            help,
        }
    }

    #[must_use]
    pub fn missing(
        path: &str,
        content: String,
        problem: Option<(usize, usize)>,
        missing: &str,
        function: &str,
        available: &[String],
    ) -> Self {
        let available = available
            .iter()
            .map(|p| format!("  • {p}"))
            .collect::<Vec<_>>()
            .join("\n");
        let help = format!(
            "Parameter '{missing}' is required for function '{function}'.\n\nExpected parameters for this function:\n{available}\n\nTip: Add the missing parameter '{missing}' to the \"parameters\" object in your test case."
        );

        Self::Parameter {
            code: NamedSource::new(path, content).with_language(language::json()),
            span: problem.map(|(start, len)| SourceSpan::new(start.into(), len)),
            help,
        }
    }

    #[must_use]
    pub fn extraneous(
        path: &str,
        content: String,
        problem: Option<(usize, usize)>,
        extra: &str,
        function: &str,
        expected: &[String],
    ) -> Self {
        let suggestion = similarity::nearest(extra, expected).unwrap_or_default();

        let expected = expected
            .iter()
            .map(|p| format!("  • {p}"))
            .collect::<Vec<_>>()
            .join("\n");
        let help = format!(
            "Parameter '{extra}' is not expected by function '{function}'.\n\nExpected parameters for this function:\n{expected}{suggestion}\n\nTip: Remove the extra parameter '{extra}' from your test case or check the function signature."
        );

        Self::Parameter {
            code: NamedSource::new(path, content).with_language(language::json()),
            span: problem.map(|(start, len)| SourceSpan::new(start.into(), len)),
            help,
        }
    }

    #[must_use]
    pub fn test(
        case: &str,
        content: String,
        span: Option<(usize, usize)>,
        issue: &str,
        suggestion: &str,
    ) -> Self {
        let help = format!("Test case issue: {issue}\n\nSuggestion: {suggestion}");

        Self::Case {
            code: NamedSource::new(case, content).with_language(language::json()),
            span: span.map(|(start, len)| SourceSpan::new(start.into(), len)),
            help,
        }
    }

    #[must_use]
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
            "Parameter '{parameter}' expects type '{expected}' but got {actual}.\n\nContext: {context}\n\nTip: Check that the value matches the expected Rust type."
        );

        Self::Type {
            code: NamedSource::new(path, content).with_language(language::json()),
            span: span.map(|(start, len)| SourceSpan::new(start.into(), len)),
            help,
        }
    }

    #[must_use]
    pub fn deserialization(
        target: &str,
        source: impl AsRef<std::path::Path>,
        json: impl Into<String>,
        error: serde_json::Error,
    ) -> Self {
        let json = json.into();
        let source = source.as_ref();

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
            source.display(),
            target
        );

        Self::Deserialization {
            json: NamedSource::new(source.to_string_lossy(), json).with_language(language::json()),
            location: SourceSpan::new(offset.into(), 1),
            help,
            cause: error,
        }
    }
}
pub type Result<T> = miette::Result<T, Error>;
