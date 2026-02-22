use clap::{Parser, ValueEnum};
use component::generation::rust::error::Error;
use component::generation::rust::schema::Cases;
use miette::{Context, IntoDiagnostic};
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
    #[arg(long)]
    template: PathBuf,
    #[arg(long)]
    cases: PathBuf,
    #[arg(long)]
    language: Language,
    #[arg(long)]
    output: PathBuf,
    #[arg(long)]
    prefix: String,
}

fn main() -> miette::Result<()> {
    command::execute(
        |_| {
            command::activate(trace::initialize(None, |channels| {
                trace::channel::Channel::matches(channels, &["generation"])
            }))
        },
        |arguments| match generate(arguments) {
            Ok(()) => Ok(()),
            Err(report) => {
                tracing::error!("{report}");
                Err(report)
            }
        },
        |result| {
            trace::flush();
            result
        },
    )
}

fn generate(arguments: Arguments) -> miette::Result<()> {
    let template = fs::read_to_string(&arguments.template)
        .into_diagnostic()
        .wrap_err(format!(
            "failed to read template: {}",
            arguments.template.display()
        ))?;

    let cases = fs::read_to_string(&arguments.cases)
        .into_diagnostic()
        .wrap_err(format!(
            "failed to read cases: {}",
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
                "failed to create output directory: {}",
                parent.display()
            ))?;
    }

    fs::write(&arguments.output, &output)
        .into_diagnostic()
        .wrap_err(format!(
            "failed to write output: {}",
            arguments.output.display()
        ))?;

    let resolved = symlink::resolve(&arguments.output, &arguments.prefix)?;
    tracing::info!("generated: {}", resolved.display());
    Ok(())
}
