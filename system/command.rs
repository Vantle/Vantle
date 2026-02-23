use clap::Parser;

pub fn activate<E>(result: Result<(), E>) -> miette::Result<()>
where
    E: miette::Diagnostic + Send + Sync + 'static,
{
    Ok(result?)
}

pub fn execute<A, I, F, D>(activation: I, run: F, deactivation: D) -> miette::Result<()>
where
    A: Parser,
    I: FnOnce(&A) -> miette::Result<()>,
    F: FnOnce(A) -> miette::Result<()>,
    D: FnOnce(miette::Result<()>) -> miette::Result<()>,
{
    dispatch(
        A::parse_from(command::arguments()?),
        activation,
        run,
        deactivation,
    )
}

pub fn dispatch<A, I, F, D>(
    arguments: A,
    activation: I,
    run: F,
    deactivation: D,
) -> miette::Result<()>
where
    I: FnOnce(&A) -> miette::Result<()>,
    F: FnOnce(A) -> miette::Result<()>,
    D: FnOnce(miette::Result<()>) -> miette::Result<()>,
{
    activation(&arguments)?;
    deactivation(run(arguments))
}
