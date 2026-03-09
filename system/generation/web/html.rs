use std::path::PathBuf;

use clap::Parser;
use observe::trace;
use page::Page;
use style::Style;

#[derive(Parser)]
#[command(
    name = "document",
    about = "Generate a document from a Rust DSL page definition"
)]
pub struct Arguments {
    #[arg(long, help = "Output file path")]
    pub output: PathBuf,

    #[arg(long, help = "Root-relative path prefix")]
    pub root: String,

    #[command(flatten)]
    pub observation: observation::argument::Argument,
}

pub fn execute<F>(run: F) -> miette::Result<()>
where
    F: FnOnce(&Arguments) -> miette::Result<PathBuf>,
{
    command::execute(
        |arguments: &Arguments| observation::initialize(&arguments.observation.sink),
        |arguments, _runtime| {
            let path = run(&arguments)?;
            println!("{}", path.display());
            Ok(())
        },
    )
}

#[trace(channels = [document])]
pub fn render(page: &mut Page) -> miette::Result<String> {
    let serializer = serialize::page(page, colorize)?;
    Ok(serializer.html)
}

#[trace(channels = [document])]
#[must_use]
pub fn css(style: &Style) -> String {
    serialize::css(style)
}

#[trace(channels = [document])]
pub fn generate(arguments: &Arguments, mut page: page::Page) -> miette::Result<PathBuf> {
    let html = render(&mut page)?;
    emit(&arguments.output, &html)?;
    Ok(arguments.output.clone())
}

#[trace(channels = [document])]
pub fn stylesheet(arguments: &Arguments, style: &Style) -> miette::Result<PathBuf> {
    emit(&arguments.output, &css(style))?;
    Ok(arguments.output.clone())
}

#[trace(channels = [document])]
fn emit(path: &std::path::Path, content: &str) -> miette::Result<()> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(|source| error::Error::Output {
            path: parent.display().to_string(),
            source,
        })?;
    }

    std::fs::write(path, content).map_err(|source| error::Error::Output {
        path: path.display().to_string(),
        source,
    })?;

    Ok(())
}

fn colorize(code: &str, language: language::Language) -> miette::Result<String> {
    highlight::highlight(code, language)
}
