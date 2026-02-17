use std::net::{IpAddr, SocketAddr};
use std::path::PathBuf;
use std::time::{Duration, Instant};

use axum::Router;
use axum::extract::Request;
use axum::middleware::Next;
use clap::Parser;
use miette::{Diagnostic, Result};
use observe::trace;
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
    Run {
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
    directory: PathBuf,

    #[arg(long, default_value = "127.0.0.1")]
    address: IpAddr,

    #[arg(long, default_value_t = 3000)]
    port: u16,
}

fn elapsed(duration: Duration) -> String {
    let microseconds = duration.as_micros();
    if microseconds < 1_000 {
        format!("{microseconds}us")
    } else if microseconds < 1_000_000 {
        format!("{:.1}ms", duration.as_secs_f64() * 1_000.0)
    } else {
        format!("{:.2}s", duration.as_secs_f64())
    }
}

#[trace(channels = [serve])]
async fn journal(request: Request, next: Next) -> axum::response::Response {
    let method = request.method().clone();
    let uri = request.uri().clone();
    let start = Instant::now();
    let response = next.run(request).await;
    let status = response.status().as_u16();
    let duration = elapsed(start.elapsed());
    if let Some(query) = uri.query() {
        info!("{status} {method} {}?{query} ({duration})", uri.path());
    } else {
        info!("{status} {method} {} ({duration})", uri.path());
    }
    response
}

#[trace(channels = [serve])]
async fn run(arguments: Arguments) -> Result<()> {
    let address = SocketAddr::new(arguments.address, arguments.port);
    let application = Router::new()
        .fallback_service(ServeDir::new(&arguments.directory))
        .layer(axum::middleware::from_fn(journal))
        .layer(TraceLayer::new_for_http());
    let listener = tokio::net::TcpListener::bind(address)
        .await
        .map_err(|source| Error::Bind { address, source })?;
    info!("http://{address}");
    axum::serve(listener, application)
        .with_graceful_shutdown(shutdown())
        .await
        .map_err(|source| Error::Run { source })?;
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
