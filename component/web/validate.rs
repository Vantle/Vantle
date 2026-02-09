use std::path::PathBuf;

use clap::Parser;
use miette::{Diagnostic, NamedSource, SourceSpan};
use thiserror::Error;
use tracing::{error, error_span};

#[derive(Parser)]
#[command(name = "validate", about = "Render W3C validation results")]
struct Arguments {
    #[arg(long)]
    source: PathBuf,

    #[arg(long)]
    report: PathBuf,

    #[arg(long)]
    output: PathBuf,

    #[arg(long)]
    prefix: String,
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

    #[error("{count} W3C validation {noun} in {destination}")]
    #[diagnostic(code(w3c::failed), help("see individual errors above for details"))]
    Failed {
        count: usize,
        noun: String,
        destination: String,
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
    tracing_subscriber::fmt::init();

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

    let arguments = Arguments::parse();
    let destination = symlink::resolve(&arguments.source, &arguments.prefix)?;
    let destination = destination.display().to_string();
    let _span = error_span!("validate", destination = %destination).entered();

    let content = std::fs::read_to_string(&arguments.source).map_err(|source| Error::Read {
        path: arguments.source.display().to_string(),
        source,
    })?;

    let json = std::fs::read_to_string(&arguments.report).map_err(|source| Error::Read {
        path: arguments.report.display().to_string(),
        source,
    })?;

    let report = serde_json::from_str::<Report>(&json).map_err(|source| Error::Parse { source })?;

    let errors = report
        .messages
        .iter()
        .filter(|m| m.kind == "error")
        .collect::<Vec<_>>();

    if errors.is_empty() {
        std::fs::write(&arguments.output, format!("VALID: {destination}\n")).map_err(|source| {
            Error::Write {
                path: arguments.output.display().to_string(),
                source,
            }
        })?;
        return Ok(());
    }

    let spans = errors.iter().map(|e| span(&content, e)).collect::<Vec<_>>();
    let source = NamedSource::new(&destination, content).with_language("html");

    for (error, s) in errors.iter().zip(spans) {
        let diagnostic = Error::Validation {
            html: source.clone(),
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
        destination,
    })?
}
