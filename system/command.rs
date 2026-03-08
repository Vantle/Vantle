pub use error;

use assemble::Assemble;
use clap::Parser;

pub fn execute<A, I, T, F>(activation: I, run: F) -> miette::Result<()>
where
    A: Parser,
    I: FnOnce(&A) -> miette::Result<T>,
    F: FnOnce(A, &pool::Schedule) -> miette::Result<()>,
{
    let query = Query::parse(command::arguments()?)?;
    let arguments: A =
        A::try_parse_from(query.arguments.clone()).map_err(|message| error::Error::Argument {
            message: message.to_string(),
            help: A::command().render_long_help().to_string(),
        })?;
    if query.active {
        return query.emit();
    }
    let schedule = pool::Assembler::new().assemble()?;
    let _context = schedule.context();
    dispatch(arguments, &schedule, activation, run)
}

struct Query {
    active: bool,
    label: Option<String>,
    output: Option<std::path::PathBuf>,
    arguments: Vec<String>,
}

impl Query {
    fn parse(mut arguments: Vec<String>) -> error::Result<Self> {
        let active = arguments.get(1).is_some_and(|a| a == "?");
        let mut label = None;
        let mut output = None;
        if active {
            arguments.remove(1);
            label = Self::extract(&mut arguments, "--label")?;
            output = Self::extract(&mut arguments, "--output")?.map(std::path::PathBuf::from);
        }
        Ok(Self {
            active,
            label,
            output,
            arguments,
        })
    }

    fn extract(arguments: &mut Vec<String>, flag: &str) -> error::Result<Option<String>> {
        let Some(position) = arguments.iter().position(|a| a == flag) else {
            return Ok(None);
        };
        arguments.remove(position);
        if position >= arguments.len() || arguments[position].starts_with("--") {
            return Err(error::Error::Flag {
                flag: flag.to_owned(),
            });
        }
        Ok(Some(arguments.remove(position)))
    }

    fn emit(&self) -> miette::Result<()> {
        let trailing = self.arguments[1..].join(" ");
        let command = match &self.label {
            Some(label) if trailing.is_empty() => format!("bazel run {label}"),
            Some(label) => format!("bazel run {label} -- {trailing}"),
            None => trailing,
        };
        match &self.output {
            Some(path) => {
                std::fs::write(path, &command).map_err(|source| error::Error::Output {
                    path: path.display().to_string(),
                    source,
                })?;
            }
            None => println!("{command}"),
        }
        Ok(())
    }
}

pub fn dispatch<A, S, I, T, F>(
    arguments: A,
    runtime: &S,
    activation: I,
    run: F,
) -> miette::Result<()>
where
    S: concurrent::Schedule,
    I: FnOnce(&A) -> miette::Result<T>,
    F: FnOnce(A, &S) -> miette::Result<()>,
{
    let _guard = activation(&arguments)?;
    run(arguments, runtime)
}
