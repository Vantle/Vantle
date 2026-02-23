use std::net::{IpAddr, SocketAddr};
use std::path::PathBuf;

use axum::Router;
use clap::Parser;
use miette::{Diagnostic, Result};
use observe::trace;
use platform::run;
use record::info;
use thiserror::Error;
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;

#[derive(Error, Debug, Diagnostic)]
pub enum Error {
    #[error("failed to bind to {address}")]
    #[diagnostic(code(serve::bind), help("check that the address is available"))]
    Bind {
        address: SocketAddr,
        #[source]
        source: std::io::Error,
    },

    #[error("server error")]
    #[diagnostic(code(serve::run), help("check file permissions in served directory"))]
    Serve {
        #[source]
        source: std::io::Error,
    },

    #[error(transparent)]
    #[diagnostic(transparent)]
    Runtime(#[from] runtime::error::Error),

    #[error(transparent)]
    #[diagnostic(transparent)]
    Trace(#[from] trace::error::Error),
}

#[derive(Parser)]
#[command(name = "serve")]
#[command(about = "Serve static files over HTTP")]
struct Arguments {
    #[arg(long, default_value = ".")]
    input: PathBuf,

    #[arg(long, default_value = "127.0.0.1")]
    address: IpAddr,

    #[arg(long, default_value_t = 3000)]
    port: u16,
}

#[trace(channels = [serve])]
async fn run(arguments: Arguments) -> Result<()> {
    let input =
        run::directory().map_or_else(|| arguments.input.clone(), |d| d.join(&arguments.input));
    let address = SocketAddr::new(arguments.address, arguments.port);
    let application = Router::new()
        .fallback_service(ServeDir::new(&input))
        .layer(TraceLayer::new_for_http());
    let listener = tokio::net::TcpListener::bind(address)
        .await
        .map_err(|source| Error::Bind { address, source })?;
    info!("http://{address}");
    axum::serve(listener, application)
        .with_graceful_shutdown(shutdown())
        .await
        .map_err(|source| Error::Serve { source })?;
    Ok(())
}

#[trace(channels = [serve])]
async fn shutdown() {
    tokio::signal::ctrl_c().await.ok();
}

fn main() -> Result<()> {
    command::execute(
        |_| {
            command::activate(trace::initialize(None, |channels| {
                trace::channel::Channel::matches(channels, &["serve"])
            }))
        },
        |arguments| runtime::global()?.block_on(run(arguments)),
        |result| {
            trace::flush();
            result
        },
    )
}
