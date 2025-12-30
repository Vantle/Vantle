pub use channel::Channel;
pub use error;
pub use tracing;
pub use translate;

use std::sync::atomic::{AtomicBool, AtomicU8, Ordering};
use std::sync::OnceLock;
use std::time::{Duration, Instant};
use tracing_subscriber::prelude::*;
use url::Url;

use proto::observation as wire;
use tokio::sync::mpsc::Receiver;
use wire::sink_client::SinkClient;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum Status {
    Starting = 0,
    Connected = 1,
    Disconnected = 2,
    Failed = 3,
}

impl From<u8> for Status {
    fn from(value: u8) -> Self {
        match value {
            0 => Status::Starting,
            1 => Status::Connected,
            2 => Status::Disconnected,
            _ => Status::Failed,
        }
    }
}

static INITIALIZED: AtomicBool = AtomicBool::new(false);
static STATUS: AtomicU8 = AtomicU8::new(Status::Starting as u8);
static ADDRESS: OnceLock<String> = OnceLock::new();

pub fn initialize<F>(address: Option<&str>, predicate: F) -> error::Result<()>
where
    F: Fn(&[Channel]) -> bool + Send + Sync + 'static,
{
    if INITIALIZED.swap(true, Ordering::SeqCst) {
        return Ok(());
    }

    let endpoint = parse(address)?;
    ADDRESS.get_or_init(|| endpoint.clone());

    let (layer, receiver) = layer::Streamer::assembler(predicate).assemble();

    tracing_subscriber::registry()
        .with(configuration::LEVEL)
        .with(tracing_subscriber::fmt::layer().with_ansi(true))
        .with(layer)
        .try_init()
        .map_err(|e| error::Error::Subscriber {
            details: e.to_string(),
        })?;

    let rt = runtime::global().map_err(|e| error::Error::Runtime {
        details: format!("{e}"),
    })?;
    let token = runtime::token().clone();
    rt.spawn(relay(endpoint, receiver, token));

    Ok(())
}

async fn relay(
    endpoint: String,
    mut receiver: Receiver<stream::Update>,
    token: tokio_util::sync::CancellationToken,
) {
    let Ok(mut client) = SinkClient::connect(endpoint.clone()).await else {
        tracing::error!("Failed to connect to observation sink: {endpoint}");
        STATUS.store(Status::Failed as u8, Ordering::SeqCst);
        return;
    };

    STATUS.store(Status::Connected as u8, Ordering::SeqCst);

    let generated = async_stream::stream! {
        loop {
            tokio::select! {
                () = token.cancelled() => break,
                received = receiver.recv() => {
                    match received {
                        Some(update) => yield convert(update),
                        None => break,
                    }
                }
            }
        }
    };

    if client.send(generated).await.is_ok() {
        STATUS.store(Status::Disconnected as u8, Ordering::SeqCst);
    } else {
        STATUS.store(Status::Failed as u8, Ordering::SeqCst);
    }
}

fn parse(address: Option<&str>) -> error::Result<String> {
    match address {
        Some(input) => {
            let parsed = Url::parse(input).map_err(|source| error::Error::Parse {
                address: input.to_string(),
                source,
            })?;

            match parsed.scheme() {
                "grpc" => {
                    let host = parsed.host_str().ok_or_else(|| error::Error::Host {
                        address: input.to_string(),
                    })?;

                    let port = parsed.port().ok_or_else(|| error::Error::Port {
                        address: input.to_string(),
                    })?;

                    Ok(format!("http://{host}:{port}"))
                }
                scheme => Err(error::Error::Scheme {
                    scheme: scheme.to_string(),
                }),
            }
        }
        None => Ok("http://127.0.0.1:50051".to_string()),
    }
}

fn convert(received: stream::Update) -> wire::Update {
    match received {
        stream::Update::Span(span) => wire::Update {
            payload: Some(wire::update::Payload::Span(translate::span(span))),
        },
        stream::Update::Event(event) => wire::Update {
            payload: Some(wire::update::Payload::Event(translate::event(event))),
        },
        stream::Update::Snapshot(snapshot) => wire::Update {
            payload: Some(wire::update::Payload::Snapshot(translate::snapshot(
                snapshot,
            ))),
        },
    }
}

#[must_use]
pub fn path() -> Option<String> {
    ADDRESS.get().cloned()
}

pub fn flush() {
    runtime::shutdown();
    let deadline = Instant::now() + Duration::from_secs(5);
    while Instant::now() < deadline {
        match status() {
            Status::Disconnected | Status::Failed => break,
            _ => std::thread::sleep(Duration::from_millis(10)),
        }
    }
}

#[must_use]
pub fn status() -> Status {
    Status::from(STATUS.load(Ordering::SeqCst))
}
