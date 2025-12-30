use clap::{Parser, Subcommand};
use component::graph::index::Index;
use component::graph::symbolic::constructor::Source;
use constructor::Constructor;
use inquire::{
    Text, set_global_render_config,
    ui::{Color, RenderConfig, StyleSheet, Styled},
};
use miette::{Diagnostic, Result};
use observe::trace;
use record::info;
use thiserror::Error;

#[ctor::ctor]
fn initialize() {
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
    #[error("Unable to evaluate input")]
    #[diagnostic(
        code(forge::evaluate),
        help("check that the input is valid Molten syntax")
    )]
    Evaluate,

    #[error(transparent)]
    #[diagnostic(transparent)]
    Arena(#[from] arena::error::Error),

    #[error(transparent)]
    #[diagnostic(transparent)]
    Trace(#[from] trace::error::Error),
}

#[derive(Parser)]
#[command(name = "forge")]
#[command(about = "Molten compiler and runtime", long_about = None)]
#[command(version)]
struct Arguments {
    #[command(subcommand)]
    command: Command,
}

#[derive(clap::Args)]
struct Sink {
    #[arg(
        long,
        help = "Stream observation data to address (e.g., file:///tmp/trace.jsonl)"
    )]
    address: Option<String>,
}

#[derive(Subcommand)]
enum Command {
    #[command(about = "Lava runtime for Molten")]
    Lava(Sink),
}

fn main() -> Result<()> {
    let Command::Lava(sink) = Arguments::parse().command;
    trace::initialize(sink.address.as_deref(), |channels| {
        channels.iter().any(|c| c.name == "core")
    })?;
    let result = lava();
    trace::flush();
    result
}

#[trace(channels = [core])]
fn process(input: String, index: &mut Index<String>) -> Result<()> {
    let module = match Source::string(input).module() {
        Ok(module) => module,
        Err(parsing) => {
            record::error!("{:?}", miette::Report::new(parsing));
            return Ok(());
        }
    };

    let (_label, _signal) = graph::index::Index::allocate(index, module)?;

    Ok(())
}

#[trace(channels = [core])]
fn lava() -> Result<()> {
    info!("Forge::Lava");

    let mut index: Index<String> = Index::default();

    loop {
        let source = Text::new(">").with_help_message("Molten.lava");

        match source.prompt() {
            Ok(input) => {
                if input.trim().is_empty() {
                    continue;
                }

                process(input, &mut index)?;
            }
            Err(_) => {
                break;
            }
        }
    }

    Ok(())
}
