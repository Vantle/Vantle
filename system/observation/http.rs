use std::net::SocketAddr;
use std::path::PathBuf;

use axum::Router;
use tokio_util::sync::CancellationToken;
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;
use tracing::Instrument;
use url::Url;

pub fn spawn(
    url: Url,
    cancellation: CancellationToken,
) -> error::Result<tokio::task::JoinHandle<()>> {
    let host = url.host_str().unwrap_or("127.0.0.1").to_string();
    let port = url.port().unwrap_or(3000);
    let address: SocketAddr = format!("{host}:{port}")
        .parse()
        .map_err(|_| error::Error::Host {
            address: url.to_string(),
        })?;

    let root = platform::run::directory().unwrap_or_else(|| PathBuf::from("."));
    let path = url.path();
    let directory = if path.is_empty() || path == "/" {
        root
    } else {
        root.join(path.trim_start_matches('/'))
    };

    let span = tracing::info_span!("http", channels = "http");

    let handle = tokio::spawn(
        async move {
            let application = Router::new()
                .fallback_service(ServeDir::new(&directory))
                .layer(TraceLayer::new_for_http());

            let listener = match tokio::net::TcpListener::bind(address).await {
                Ok(listener) => listener,
                Err(e) => {
                    tracing::error!("failed to bind to {address}: {e}");
                    return;
                }
            };

            tracing::info!("serving {} at http://{address}", directory.display());

            let shutdown = cancellation.cancelled_owned();
            if let Err(e) = axum::serve(listener, application)
                .with_graceful_shutdown(shutdown)
                .await
            {
                tracing::error!("server error: {e}");
            }
        }
        .instrument(span),
    );

    Ok(handle)
}
