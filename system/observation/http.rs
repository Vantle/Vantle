use std::net::SocketAddr;
use std::path::PathBuf;

use axum::Router;
use axum::extract::Request;
use axum::middleware;
use axum::response::Response;
use tokio_util::sync::CancellationToken;
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;
use tracing::Instrument;
use url::Url;

pub struct Server {
    url: Url,
    cancellation: CancellationToken,
}

impl Server {
    #[must_use]
    pub fn new(url: Url, cancellation: CancellationToken) -> Self {
        Self { url, cancellation }
    }

    pub fn spawn(self) -> error::Result<tokio::task::JoinHandle<()>> {
        let host = self.url.host_str().unwrap_or("127.0.0.1").to_string();
        let port = self.url.port().unwrap_or(3000);
        let address: SocketAddr =
            format!("{host}:{port}")
                .parse()
                .map_err(|_| error::Error::Host {
                    address: self.url.to_string(),
                })?;

        let root = platform::run::directory().unwrap_or_else(|| PathBuf::from("."));
        let path = self.url.path();
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
                    .layer(middleware::from_fn(nocache))
                    .layer(TraceLayer::new_for_http());

                let listener = match tokio::net::TcpListener::bind(address).await {
                    Ok(listener) => listener,
                    Err(e) => {
                        tracing::error!("failed to bind to {address}: {e}");
                        return;
                    }
                };

                tracing::info!("serving {} at http://{address}", directory.display());

                let shutdown = self.cancellation.cancelled_owned();
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
}

async fn nocache(request: Request, next: middleware::Next) -> Response {
    let mut response = next.run(request).await;
    response
        .headers_mut()
        .insert("cache-control", "no-store".parse().unwrap());
    response
}
