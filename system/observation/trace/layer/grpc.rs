use assemble::Assemble;
use std::sync::Arc;
use stream::Predicate;
use tokio::sync::mpsc::Receiver;
use tokio_util::sync::CancellationToken;
use url::Url;

pub fn layer(
    predicate: Predicate,
    backpressure: layer::Backpressure,
) -> (layer::Streamer, Receiver<stream::Update>) {
    layer::Streamer::assembler(predicate)
        .backpressure(backpressure)
        .assemble()
}

pub fn spawn(
    url: Url,
    receiver: Receiver<stream::Update>,
    cancellation: CancellationToken,
) -> error::Result<tokio::task::JoinHandle<()>> {
    Ok(tokio::spawn(relay(url, receiver, cancellation)))
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
}
