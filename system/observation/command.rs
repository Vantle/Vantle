use clap::Parser;
use concurrent::Schedule;
use miette::Result;

#[derive(Parser)]
#[command(name = "serve", about = "Serve static files over HTTP")]
struct Arguments {
    #[command(flatten)]
    observation: observation::argument::Argument,
}

fn main() -> Result<()> {
    command::execute(
        |arguments: &Arguments| observation::initialize(&arguments.observation.sink),
        |_, runtime| {
            runtime.block(async {
                tokio::signal::ctrl_c().await.ok();
            });
            Ok(())
        },
    )
}
