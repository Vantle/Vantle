use clap::{Parser, Subcommand};
use component::graph::arena::{error::allocation, Valued};
use component::graph::attribute::Attribute;
use component::graph::matrix::Related;
use component::graph::state::{Inference, Wave};
use constructor::Constructor;
use evaluation::evaluator::lava;
use inquire::{
    set_global_render_config,
    ui::{Color, RenderConfig, StyleSheet, Styled},
    Text,
};
use logging::{error, info};
use miette::{Diagnostic, NamedSource, Report, Result};
use std::io::Cursor;
use symbolic::renderer;
use system::language;
use thiserror::Error;

#[ctor::ctor]
fn initialize() {
    logging::configure(log::LevelFilter::Info);

    let configuration = RenderConfig::default_colored()
        .with_prompt_prefix(Styled::new("🔥").with_fg(Color::LightRed))
        .with_answered_prompt_prefix(Styled::new("🔥").with_fg(Color::LightRed))
        .with_help_message(StyleSheet::new().with_fg(Color::LightCyan))
        .with_text_input(StyleSheet::new().with_fg(Color::White))
        .with_answer(StyleSheet::new().with_fg(Color::LightGreen))
        .with_highlighted_option_prefix(Styled::new("▶").with_fg(Color::LightMagenta))
        .with_selected_checkbox(Styled::new("✓").with_fg(Color::LightGreen))
        .with_unselected_checkbox(Styled::new("○").with_fg(Color::DarkGrey))
        .with_canceled_prompt_indicator(Styled::new("✗").with_fg(Color::LightRed))
        .with_error_message(
            inquire::ui::ErrorMessageRenderConfig::default_colored()
                .with_prefix(Styled::new("⚠").with_fg(Color::LightRed))
                .with_message(StyleSheet::new().with_fg(Color::LightRed)),
        );
    set_global_render_config(configuration);
}

#[derive(Error, Debug, Diagnostic)]
pub enum Error {
    #[error("Unable to parse input")]
    #[diagnostic(code(forge::parse))]
    Parse {
        #[source_code]
        code: NamedSource<String>,
        #[diagnostic_source]
        source: constructor::Error,
    },

    #[error(transparent)]
    #[diagnostic(transparent)]
    Evaluate(#[from] lava::Error),

    #[error(transparent)]
    #[diagnostic(transparent)]
    State(#[from] allocation::Allocation),
}

impl Error {
    pub fn construction(input: String, error: constructor::Error) -> Self {
        Self::Parse {
            code: NamedSource::new("input", input).with_language(language::molten()),
            source: error,
        }
    }
}

#[derive(Parser)]
#[command(name = "forge")]
#[command(about = "Molten compiler and runtime", long_about = None)]
#[command(version)]
struct Arguments {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    #[command(about = "Temporal runtime for Molten")]
    Temporal,
}

fn main() -> Result<()> {
    let arguments = Arguments::parse();

    match arguments.command {
        Command::Temporal => temporal(),
    }
}

fn temporal() -> Result<()> {
    info!("Forge::Temporal");

    let mut arena: Valued<Attribute<String>> = Valued::none();
    let mut relations: Related<Wave<usize>> = Related::none();
    let mut inference: Inference<usize> = Inference::new();

    loop {
        let source = Text::new("").with_help_message("Molten.lava");

        match source.prompt() {
            Ok(input) => {
                if input.trim().is_empty() {
                    continue;
                }

                match Cursor::new(input.as_bytes()).module() {
                    Ok(module) => {
                        arena.insert(module.clone())?;
                        graph::mutator::relate(&module, &arena, &mut relations);
                        let signal = graph::constructor::signal(&module, &arena);
                        lava::propagate(&mut inference, &signal, &relations, None)?;
                        info!("{}", renderer::inference(&arena, &inference)?);
                    }
                    Err(parsing) => {
                        error!("{:?}", Report::new(Error::construction(input, parsing)));
                        continue;
                    }
                }
            }
            Err(_) => {
                break;
            }
        }
    }

    Ok(())
}
