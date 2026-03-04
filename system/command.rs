use clap::Parser;
use miette::IntoDiagnostic;

pub fn activate<T, E>(result: Result<T, E>) -> miette::Result<T>
where
    E: miette::Diagnostic + Send + Sync + 'static,
{
    Ok(result?)
}

pub fn execute<A, I, T, F>(activation: I, run: F) -> miette::Result<()>
where
    A: Parser,
    I: FnOnce(&A) -> miette::Result<T>,
    F: FnOnce(A) -> miette::Result<()>,
{
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(64)
        .thread_name("concurrent")
        .enable_all()
        .build()
        .into_diagnostic()?;
    let _context = runtime.enter();
    dispatch(A::parse_from(command::arguments()?), activation, run)
}

pub fn dispatch<A, I, T, F>(arguments: A, activation: I, run: F) -> miette::Result<()>
where
    I: FnOnce(&A) -> miette::Result<T>,
    F: FnOnce(A) -> miette::Result<()>,
{
    let _guard = activation(&arguments)?;
    run(arguments)
}
