use clap::Parser;

#[macro_export]
macro_rules! output {
    ($help:literal) => {
        #[derive(clap::Parser)]
        pub struct Output {
            #[arg(long = "output", help = $help)]
            pub path: std::path::PathBuf,
        }
    };
}

#[macro_export]
macro_rules! prefix {
    () => {
        #[derive(clap::Parser)]
        pub struct Prefix {
            #[arg(long = "prefix", help = "Bazel symlink prefix for path resolution")]
            pub value: String,
        }
    };
}

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
    let arguments = A::parse();
    activation(&arguments)?;
    deactivation(run(arguments))
}
