use std::path::PathBuf;

use clap::Parser;
use miette::{Context, Diagnostic, IntoDiagnostic};
use thiserror::Error;

#[derive(Error, Diagnostic, Debug)]
enum Error {
    #[error("no execution json found in data paths")]
    #[diagnostic(
        code(measurement::missing::execution),
        help("provide a .json file via --data")
    )]
    Execution,
}

#[derive(Parser)]
#[command(
    name = "measurement",
    about = "Generate performance visualization card library from execution results"
)]
struct Arguments {
    #[arg(long)]
    output: PathBuf,

    #[arg(long)]
    data: Vec<PathBuf>,

    #[command(flatten)]
    observation: observation::argument::Argument,
}

fn main() -> miette::Result<()> {
    command::execute(
        |arguments: &Arguments| observation::initialize(&arguments.observation.sink),
        |arguments, _runtime| {
            let execution = arguments
                .data
                .iter()
                .find(|p| p.extension().is_some_and(|e| e == "json"))
                .ok_or(Error::Execution)?;

            let content = std::fs::read_to_string(execution)
                .into_diagnostic()
                .wrap_err(format!("failed to read execution: {}", execution.display()))?;

            let ast: syn::File = syn::parse_quote! {
                #[must_use]
                pub fn cards() -> Vec<card::Group> {
                    performance::groups(
                        serde_json::from_str::<performance::Report>(#content)
                            .expect("valid performance execution json"),
                    )
                }
            };

            let output = prettyplease::unparse(&ast);

            if let Some(parent) = arguments.output.parent() {
                std::fs::create_dir_all(parent)
                    .into_diagnostic()
                    .wrap_err(format!("failed to create directory: {}", parent.display()))?;
            }

            std::fs::write(&arguments.output, output)
                .into_diagnostic()
                .wrap_err(format!(
                    "failed to write output: {}",
                    arguments.output.display()
                ))?;

            Ok(())
        },
    )
}
