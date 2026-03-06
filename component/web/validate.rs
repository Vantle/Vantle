use std::path::PathBuf;

use clap::{Parser, ValueEnum};
use miette::{Diagnostic, NamedSource, SourceSpan};
use thiserror::Error;
use tracing::{error, error_span};

#[derive(Debug, Clone, ValueEnum)]
enum Kind {
    Html,
    Css,
    Svg,
}

impl std::fmt::Display for Kind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Html => write!(f, "html"),
            Self::Css => write!(f, "css"),
            Self::Svg => write!(f, "svg"),
        }
    }
}

#[derive(Parser)]
#[command(name = "validate", about = "Render W3C validation results")]
struct Arguments {
    #[arg(long, help = "Source file to validate")]
    source: PathBuf,

    #[arg(long, help = "Output file for validation results")]
    output: PathBuf,

    #[arg(long, help = "Origin identifier for error reporting")]
    origin: String,

    #[arg(long, help = "Path to Java runtime")]
    java: PathBuf,

    #[arg(long, help = "Path to vnu.jar validator")]
    validator: PathBuf,

    #[arg(long, help = "File type to validate")]
    kind: Kind,

    #[command(flatten)]
    observation: observation::argument::Argument,
}

#[derive(serde::Deserialize)]
struct Report {
    messages: Vec<Message>,
}

#[derive(serde::Deserialize)]
struct Message {
    #[serde(rename = "type")]
    kind: String,
    #[serde(rename = "message")]
    text: Option<String>,
    #[serde(rename = "firstLine")]
    first_line: Option<usize>,
    #[serde(rename = "lastLine")]
    last_line: Option<usize>,
    #[serde(rename = "firstColumn")]
    first_column: Option<usize>,
    #[serde(rename = "lastColumn")]
    last_column: Option<usize>,
}

#[derive(Error, Debug, Diagnostic)]
enum Error {
    #[error("failed to read {path}")]
    #[diagnostic(code(w3c::read), help("check that the file exists and is readable"))]
    Read {
        path: String,
        #[source]
        source: std::io::Error,
    },

    #[error("failed to parse validation report")]
    #[diagnostic(
        code(w3c::parse),
        help("the validator may have produced malformed JSON")
    )]
    Parse {
        #[source]
        source: serde_json::Error,
    },

    #[error("failed to write {path}")]
    #[diagnostic(code(w3c::write), help("check directory permissions and disk space"))]
    Write {
        path: String,
        #[source]
        source: std::io::Error,
    },

    #[error("{message}")]
    #[diagnostic(code(w3c::validation))]
    Validation {
        #[source_code]
        html: NamedSource<String>,

        #[label("here")]
        span: Option<SourceSpan>,

        message: String,

        #[help]
        help: String,
    },

    #[error("{count} W3C validation {noun} in {origin}")]
    #[diagnostic(code(w3c::failed), help("see individual errors above for details"))]
    Failed {
        count: usize,
        noun: String,
        origin: String,
    },

    #[error("unrecognized file extension for {path}")]
    #[diagnostic(code(w3c::extension), help("supported extensions are .html and .css"))]
    Extension { path: String },

    #[error("failed to run validator")]
    #[diagnostic(
        code(w3c::checker),
        help("check that the Java runtime and vnu.jar are available")
    )]
    Checker {
        #[source]
        source: std::io::Error,
    },
}

fn offset(content: &str, line: usize, column: usize) -> usize {
    let mut position = 0;
    for (index, row) in content.lines().enumerate() {
        if index + 1 == line {
            return position + column.saturating_sub(1);
        }
        position += row.len() + 1;
    }
    position
}

fn span(content: &str, message: &Message) -> Option<SourceSpan> {
    let first_column = message.first_column?;
    let first_line = message.first_line.or(message.last_line)?;
    let start = offset(content, first_line, first_column);

    let length = match (message.last_line, message.last_column) {
        (Some(last_line), Some(last_column)) => {
            let end = offset(content, last_line, last_column);
            (end + 1).saturating_sub(start)
        }
        _ => 1,
    };

    Some(SourceSpan::new(start.into(), length))
}

fn main() -> miette::Result<()> {
    command::execute(
        |arguments: &Arguments| observation::initialize(&arguments.observation.sink),
        |arguments, _runtime| run(arguments),
    )
}

fn run(arguments: Arguments) -> miette::Result<()> {
    miette::set_hook(Box::new(|_| {
        Box::new(
            miette::MietteHandlerOpts::new()
                .terminal_links(false)
                .color(true)
                .unicode(true)
                .context_lines(2)
                .build(),
        )
    }))?;

    let origin = arguments.origin.clone();
    let _span = error_span!("validate", origin = %origin).entered();

    let content = std::fs::read_to_string(&arguments.source).map_err(|source| Error::Read {
        path: arguments.source.display().to_string(),
        source,
    })?;

    let checker = std::process::Command::new(&arguments.java)
        .arg("-jar")
        .arg(&arguments.validator)
        .arg("--format")
        .arg("json")
        .arg(format!("--{}", &arguments.kind))
        .arg(&arguments.source)
        .output()
        .map_err(|source| Error::Checker { source })?;

    let json = String::from_utf8_lossy(&checker.stderr);
    let report = serde_json::from_str::<Report>(&json).map_err(|source| Error::Parse { source })?;

    let errors = report
        .messages
        .iter()
        .filter(|m| m.kind == "error")
        .collect::<Vec<_>>();

    if errors.is_empty() {
        std::fs::write(&arguments.output, format!("VALID: {origin}\n")).map_err(|source| {
            Error::Write {
                path: arguments.output.display().to_string(),
                source,
            }
        })?;
        return Ok(());
    }

    let spans = errors.iter().map(|e| span(&content, e)).collect::<Vec<_>>();
    let language = arguments
        .source
        .extension()
        .and_then(|e| e.to_str())
        .ok_or_else(|| Error::Extension {
            path: arguments.source.display().to_string(),
        })?;
    let source = std::sync::Arc::new(NamedSource::new(&origin, content).with_language(language));

    for (error, s) in errors.iter().zip(spans) {
        let diagnostic = Error::Validation {
            html: (*source).clone(),
            span: s,
            message: error.text.clone().unwrap_or_default(),
            help: "see https://validator.w3.org for W3C validation rules".into(),
        };
        error!("{}", error.text.as_deref().unwrap_or_default());
        eprintln!("{:?}", miette::Report::new(diagnostic));
    }

    Err(Error::Failed {
        count: errors.len(),
        noun: if errors.len() == 1 { "error" } else { "errors" }.into(),
        origin,
    })?
}
