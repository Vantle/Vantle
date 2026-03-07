use clap::{Parser, ValueEnum};
use component::generation::rust::{error::Error, schema::Cases};
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
    #[arg(long, help = "Path to template file")]
    template: PathBuf,
    #[arg(long, help = "Path to test cases JSON file")]
    cases: PathBuf,
    #[arg(long, help = "Target language for generation")]
    language: Language,
    #[arg(long, help = "Output file path")]
    output: PathBuf,
    #[arg(long, help = "Path to performance specification JSON file")]
    specification: Option<PathBuf>,
}

fn main() -> miette::Result<()> {
    command::execute(
        |_| observation::initialize(&[]),
        |arguments, _runtime| match generate(arguments) {
            Ok(()) => Ok(()),
            Err(report) => {
                tracing::error!("{report}");
                Err(report)
            }
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
        Language::Rust => match arguments.specification {
            None => rust::generate(
                &template,
                &data,
                &cases,
                &arguments.cases.to_string_lossy(),
                &arguments.template.to_string_lossy(),
            )
            .map_err(|e| miette::Report::new(*e))?,
            Some(ref specification_path) => {
                let specification_content = fs::read_to_string(specification_path)
                    .into_diagnostic()
                    .wrap_err(format!(
                        "failed to read specification: {}",
                        specification_path.display()
                    ))?;

                let specification: performance::Specification =
                    serde_json::from_str(&specification_content).map_err(|e| {
                        Error::deserialization(
                            "Specification",
                            specification_path,
                            &specification_content,
                            e,
                        )
                    })?;

                rust::benchmark(
                    &template,
                    &data,
                    &specification,
                    &cases,
                    &arguments.cases.to_string_lossy(),
                    &arguments.template.to_string_lossy(),
                    &specification_path.to_string_lossy(),
                )
                .map_err(|e| miette::Report::new(*e))?
            }
        },
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

    Ok(())
}
