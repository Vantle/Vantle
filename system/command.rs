use assemble::Assemble;
use clap::Parser;

pub fn execute<A, I, T, F>(activation: I, run: F) -> miette::Result<()>
where
    A: Parser,
    I: FnOnce(&A) -> miette::Result<T>,
    F: FnOnce(A, &pool::Schedule) -> miette::Result<()>,
{
    let arguments = A::parse_from(command::arguments()?);
    let schedule = pool::Assembler::new().assemble()?;
    let _context = schedule.context();
    dispatch(arguments, &schedule, activation, run)
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
