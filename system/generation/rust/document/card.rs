use std::path::PathBuf;

use clap::Parser;
use miette::{Context, Diagnostic, IntoDiagnostic};
use thiserror::Error;

#[derive(Error, Diagnostic, Debug)]
enum Error {
    #[error("no execution json found in data paths")]
    #[diagnostic(
        code(card::missing::execution),
        help("provide a .json file via --data")
    )]
    Execution,

    #[error("no template rs found in data paths")]
    #[diagnostic(code(card::missing::template), help("provide a .rs file via --data"))]
    Template,
}

#[derive(Parser)]
#[command(
    name = "card",
    about = "Generate visualization card library from execution results"
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
            let (execution, template) = partition(&arguments.data)?;

            let execution_content = std::fs::read_to_string(&execution)
                .into_diagnostic()
                .wrap_err(format!("failed to read execution: {}", execution.display()))?;

            let template_content = std::fs::read_to_string(&template)
                .into_diagnostic()
                .wrap_err(format!("failed to read template: {}", template.display()))?;

            let template_path = template.display().to_string();

            let ast: syn::File = syn::parse_quote! {
                #[must_use]
                pub fn cards() -> Vec<card::Group> {
                    visualize::cards(
                        serde_json::from_str::<visualize::Execution>(#execution_content)
                            .expect("valid execution json"),
                        &[visualize::Template {
                            path: #template_path.into(),
                            content: #template_content.into(),
                        }],
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

fn partition(paths: &[PathBuf]) -> miette::Result<(PathBuf, PathBuf)> {
    let mut execution = None;
    let mut template = None;

    for path in paths {
        match path.extension().and_then(|extension| extension.to_str()) {
            Some("json") => execution = Some(path.clone()),
            Some("rs") => template = Some(path.clone()),
            _ => {}
        }
    }

    let execution = execution.ok_or(Error::Execution)?;
    let template = template.ok_or(Error::Template)?;

    Ok((execution, template))
}
