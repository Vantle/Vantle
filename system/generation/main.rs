use clap::{Parser, ValueEnum};
use component::generation::rust::error::Error;
use component::generation::rust::schema::Cases;
use miette::{IntoDiagnostic, Result};
use record::info;
use std::{fs, path::PathBuf};

#[derive(Debug, Clone, ValueEnum)]
#[clap(rename_all = "kebab-case")]
enum Language {
    Rust,
}

#[derive(Parser)]
#[command(name = "generator")]
#[command(about = "Generate test files from templates using case data")]
struct Arguments {
    #[arg(long, required = true)]
    template: PathBuf,
    #[arg(long, required = true)]
    cases: PathBuf,
    #[arg(long, required = true)]
    language: Language,
    #[arg(long, required = true)]
    output: PathBuf,
}

fn main() -> Result<()> {
    let output = run()?;
    info!(
        "✅ Generated: {} in sandbox {}",
        std::env::var("OUTPUT_PATH").unwrap_or_default(),
        output.display()
    );
    Ok(())
}

fn run() -> miette::Result<PathBuf> {
    let arguments = Arguments::parse();

    let template = fs::read_to_string(&arguments.template)
        .into_diagnostic()
        .map_err(|e| {
            e.with_source_code(format!(
                "📁 Failed to read template file: {}\n\n💡 Tip: Check that the file exists and you have read permissions.",
                arguments.template.display()
            ))
        })?;

    let cases = fs::read_to_string(&arguments.cases)
        .into_diagnostic()
        .map_err(|e| {
            e.with_source_code(format!(
                "📁 Failed to read cases file: {}\n\n💡 Tip: Check that the file exists and you have read permissions.",
                arguments.cases.display()
            ))
        })?;

    let data: Cases = serde_json::from_str(&cases)
        .map_err(|e| Error::deserialization("Cases", &arguments.cases, &cases, e))?;

    let output = match arguments.language {
        Language::Rust => {
            rust::generate(&template, &data, &cases, &arguments.cases.to_string_lossy())
                .map_err(|e| miette::Report::new(*e))?
        }
    };

    if let Some(parent) = arguments.output.parent() {
        fs::create_dir_all(parent)
            .into_diagnostic()
            .map_err(|e| {
                e.with_source_code(format!(
                    "📁 Failed to create output directory: {}\n\n💡 Tip: Check that you have write permissions to the parent directory.",
                    parent.display()
                ))
            })?;
    }

    fs::write(&arguments.output, &output)
        .into_diagnostic()
        .map_err(|e| {
            e.with_source_code(format!(
                "📁 Failed to write output file: {}\n\n💡 Tip: Check that you have write permissions to the output directory.",
                arguments.output.display()
            ))
        })?;

    Ok(arguments.output)
}
