use clap::{Parser, Subcommand};
use component::graph::arena::Valued;
use component::graph::attribute::Attribute;
use component::graph::matrix::Related;
use component::graph::state::{Inference, Wave};
use constructor::Constructor;
use evaluate::lava;
use miette::{IntoDiagnostic, Result};
use std::io::{self, BufRead, Cursor};

#[derive(Parser)]
#[command(name = "forge")]
#[command(about = "Molten runtime and interactive development environment", long_about = None)]
#[command(version)]
struct Arguments {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    #[command(about = "Interactive temporal runtime for Molten")]
    Temporal,
}

fn main() -> Result<()> {
    logging::configure(log::LevelFilter::Error);

    let arguments = Arguments::parse();

    match arguments.command {
        Command::Temporal => temporal(),
    }
}

fn temporal() -> Result<()> {
    println!("Forge::Temporal");

    let mut arena: Valued<Attribute<String>> = Valued::none();
    let mut relations: Related<Wave<usize>> = Related::none();
    let mut inference: Inference<usize> = Inference::new();

    let stdin = io::stdin();
    let mut reader = stdin.lock();
    let mut line = String::new();

    loop {
        line.clear();
        let bytes = reader.read_line(&mut line).into_diagnostic()?;

        if bytes == 0 {
            return Ok(());
        }

        let input = line.trim();

        if input.is_empty() {
            continue;
        }

        let module = match Cursor::new(input.as_bytes()).module() {
            Ok(m) => m,
            Err(e) => {
                eprintln!("Error: {}", e);
                continue;
            }
        };

        arena.insert(module.clone()).into_diagnostic()?;

        let subgraph = graph::constructor::relate(&module, &arena);
        let signal = graph::constructor::signal(&module, &arena);

        relations.merge(&subgraph);

        lava::propagate(&mut inference, &signal, &relations, None)
            .map_err(|e| eprintln!("{:?}", e))
            .map(|()| println!("{:#?}", inference))
            .ok();
    }
}
