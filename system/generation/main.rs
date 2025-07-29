use autolog::{error, info};
use clap::{Parser, ValueEnum};
use component::generation::rust::schema::Cases;
use miette::{IntoDiagnostic, Result};
use std::{fs, path::PathBuf, process};

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

fn main() {
    miette::set_hook(Box::new(|_| {
        Box::new(
            miette::MietteHandlerOpts::new()
                .terminal_links(true)
                .unicode(true)
                .context_lines(5)
                .tab_width(2)
                .color(true)
                .force_graphical(true)
                .build(),
        )
    }))
    .unwrap_or_else(|e| {
        error!("Failed to initialize miette error reporting system: {}", e);
        info!("This will affect error display quality but the program can continue.");
        info!("Consider checking your terminal capabilities or miette configuration.");
    });

    match run() {
        Ok(_output) => {
            info!(
                "✅ Generated: {}",
                std::env::var("OUTPUT_PATH").unwrap_or_default()
            );
        }
        Err(err) => {
            error!("Generation: {:?}", err);
            process::exit(1);
        }
    }
}

fn run() -> Result<PathBuf> {
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
        .into_diagnostic()
        .map_err(|e| {
            e.with_source_code(format!(
                "📄 Failed to parse JSON in cases file: {}\n\n💡 Tip: Check your JSON syntax using a JSON validator.",
                arguments.cases.display()
            ))
        })?;

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
