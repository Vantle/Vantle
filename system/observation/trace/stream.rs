pub use channel;
pub use error;
pub use tracing;

use assemble::Assemble;
use channel::Channel;
use std::sync::OnceLock;
use std::sync::atomic::{AtomicBool, AtomicU8, Ordering};
use std::time::{Duration, Instant};
use tracing_subscriber::prelude::*;
use url::Url;

use std::sync::Arc;
use tokio::sync::mpsc::Receiver;

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

    let url = parse(address)?;
    ADDRESS.get_or_init(|| url.to_string());

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
    rt.spawn(relay(url, receiver, token));

    Ok(())
}

async fn relay(
    url: Url,
    mut receiver: Receiver<stream::Update>,
    token: tokio_util::sync::CancellationToken,
) {
    let peer = Arc::new(peer::Assembler::new().address(url).assemble());

    let server = Arc::clone(&peer);
    let shutdown = token.clone();
    tokio::spawn(async move {
        tokio::select! {
            result = server.serve() => {
                if let Err(e) = result {
                    tracing::error!("Observation server failed: {e}");
                    STATUS.store(Status::Failed as u8, Ordering::SeqCst);
                }
            }
            () = shutdown.cancelled() => {}
        }
    });

    STATUS.store(Status::Connected as u8, Ordering::SeqCst);

    loop {
        tokio::select! {
            () = token.cancelled() => break,
            received = receiver.recv() => {
                match received {
                    Some(update) => {
                        let _ = peer.send(update);
                    }
                    None => break,
                }
            }
        }
    }

    STATUS.store(Status::Disconnected as u8, Ordering::SeqCst);
}

fn parse(address: Option<&str>) -> error::Result<Url> {
    let input = address.unwrap_or("grpc://127.0.0.1:50051");

    let parsed = Url::parse(input).map_err(|source| error::Error::Parse {
        address: input.to_string(),
        source,
    })?;

    match parsed.scheme() {
        "grpc" => Ok(parsed),
        scheme => Err(error::Error::Scheme {
            scheme: scheme.to_string(),
        }),
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
