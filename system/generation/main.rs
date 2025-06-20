use clap::{Parser, ValueEnum};
use component::error::Error;
use component::schema::Cases;
use std::{
    fs::{self, File},
    io::BufReader,
    path::PathBuf,
    process,
};

pub use component;
pub use rust;

#[derive(Debug, Clone, ValueEnum)]
#[clap(rename_all = "kebab-case")]
enum Language {
    Rust,
}

#[derive(Parser)]
#[command(name = "generator")]
#[command(about = "Generate test files from templates using JSON test case data")]
struct Arguments {
    #[arg(long, required = true)]
    template: PathBuf,
    #[arg(long, required = true)]
    data: PathBuf,
    #[arg(long, required = true)]
    language: Language,
    #[arg(long, required = true)]
    output: PathBuf,
}

fn main() {
    match run() {
        Ok(output) => {
            println!("{}", output.display());
        }
        Err(error) => {
            eprintln!("Error: {}", error);
            process::exit(error.code());
        }
    }
}

fn run() -> Result<PathBuf, Error> {
    let arguments = Arguments::parse();
    let template = fs::read_to_string(&arguments.template)?;
    let data: Cases = serde_json::from_reader(BufReader::new(File::open(&arguments.data)?))?;

    let output = match arguments.language {
        Language::Rust => rust::generate(&template, &data)?,
    };

    arguments
        .output
        .parent()
        .map(fs::create_dir_all)
        .transpose()
        .map_err(Error::Io)?;
    fs::write(&arguments.output, &output)?;
    Ok(arguments.output)
}
