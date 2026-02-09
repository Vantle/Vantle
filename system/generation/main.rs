use clap::{Parser, ValueEnum};
use component::generation::rust::error::Error;
use component::generation::rust::schema::Cases;
use miette::{Context, IntoDiagnostic};
use observe::trace;
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
    #[arg(long, required = true)]
    prefix: String,
}

fn main() -> miette::Result<()> {
    trace::initialize(None, |channels| {
        channels.iter().any(|c| c.name == "generation")
    })?;
    let result = run();
    trace::flush();
    result
}

#[trace(channels = [generation])]
fn run() -> miette::Result<()> {
    match generate() {
        Ok(()) => Ok(()),
        Err(report) => {
            tracing::error!("{report}");
            Err(report)
        }
    }
}

fn generate() -> miette::Result<()> {
    let arguments = Arguments::parse();

    let template = fs::read_to_string(&arguments.template)
        .into_diagnostic()
        .wrap_err(format!(
            "📁 failed to read template: {}\n\n💡 check that the file exists and you have read permissions",
            arguments.template.display()
        ))?;

    let cases = fs::read_to_string(&arguments.cases)
        .into_diagnostic()
        .wrap_err(format!(
            "📁 failed to read cases: {}\n\n💡 check that the file exists and you have read permissions",
            arguments.cases.display()
        ))?;

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
            .wrap_err(format!(
                "📁 failed to create output directory: {}\n\n💡 check that you have write permissions to the parent directory",
                parent.display()
            ))?;
    }

    fs::write(&arguments.output, &output)
        .into_diagnostic()
        .wrap_err(format!(
            "📁 failed to write output: {}\n\n💡 check that you have write permissions to the output directory",
            arguments.output.display()
        ))?;

    let resolved = symlink::resolve(&arguments.output, &arguments.prefix)?;
    tracing::info!("generated: {}", resolved.display());
    Ok(())
}
