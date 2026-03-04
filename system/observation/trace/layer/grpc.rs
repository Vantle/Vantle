use assemble::Assemble;
use std::sync::Arc;
use std::sync::atomic::{AtomicU8, Ordering};
use std::time::{Duration, Instant};
use stream::Predicate;
use tokio::sync::mpsc::Receiver;
use tokio_util::sync::CancellationToken;
use url::Url;

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

static STATUS: AtomicU8 = AtomicU8::new(Status::Starting as u8);

pub fn layer(predicate: Predicate) -> (layer::Streamer, Receiver<stream::Update>) {
    layer::Streamer::assembler(predicate).assemble()
}

pub fn spawn(
    url: Url,
    receiver: Receiver<stream::Update>,
    cancellation: CancellationToken,
) -> error::Result<()> {
    tokio::spawn(relay(url, receiver, cancellation));
    Ok(())
}

async fn relay(url: Url, mut receiver: Receiver<stream::Update>, token: CancellationToken) {
    let peer = Arc::new(peer::Assembler::new().address(url).assemble());

    let server = Arc::clone(&peer);
    let shutdown = token.clone();
    tokio::spawn(async move {
        tokio::select! {
            result = server.serve() => {
                if let Err(e) = result {
                    tracing::error!("Sink server failed: {e}");
                    STATUS.store(Status::Failed as u8, Ordering::SeqCst);
                }
            }
            () = shutdown.cancelled() => {}
        }
    });

    loop {
        tokio::select! {
            () = token.cancelled() => break,
            received = receiver.recv() => {
                match received {
                    Some(update) => {
                        if let Err(e) = peer.send(update) {
                            tracing::warn!("failed to relay observation update: {e}");
                        }
                    }
                    None => break,
                }
            }
        }
    }

    STATUS.store(Status::Disconnected as u8, Ordering::SeqCst);
}

pub fn flush() {
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
