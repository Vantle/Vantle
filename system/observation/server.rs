pub use error;

use proto::observation::sink_server::{Sink, SinkServer};
use proto::observation::source_server::{Source, SourceServer};
use proto::observation::{Acknowledge, Command, Record, Update};
use std::net::SocketAddr;
use std::path::PathBuf;
use std::pin::Pin;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use tokio::sync::broadcast;
use tokio_stream::{wrappers::BroadcastStream, Stream, StreamExt};
use tonic::{Request, Response, Status, Streaming};

type Emission = Pin<Box<dyn Stream<Item = Result<Update, Status>> + Send>>;

pub struct Assembler {
    capacity: usize,
    storage: Option<PathBuf>,
}

impl Assembler {
    #[must_use]
    pub fn capacity(mut self, capacity: usize) -> Self {
        self.capacity = capacity;
        self
    }

    #[must_use]
    pub fn storage(mut self, path: PathBuf) -> Self {
        self.storage = Some(path);
        self
    }

    #[must_use]
    pub fn assemble(self) -> Arc<Portal> {
        let (sender, _) = broadcast::channel(self.capacity);
        Arc::new(Portal {
            sender,
            storage: self.storage,
            sequence: AtomicU64::new(0),
        })
    }
}

pub struct Portal {
    sender: broadcast::Sender<Update>,
    storage: Option<PathBuf>,
    sequence: AtomicU64,
}

impl Portal {
    #[must_use]
    pub fn assembler() -> Assembler {
        Assembler {
            capacity: 4096,
            storage: None,
        }
    }
}

#[tonic::async_trait]
impl Sink for Portal {
    async fn send(
        &self,
        request: Request<Streaming<Update>>,
    ) -> Result<Response<Acknowledge>, Status> {
        let mut stream = request.into_inner();

        while let Some(result) = stream.next().await {
            match result {
                Ok(update) => {
                    let _ = self.sender.send(update);
                }
                Err(status) => {
                    return Err(status);
                }
            }
        }

        Ok(Response::new(Acknowledge {}))
    }

    async fn store(&self, request: Request<Streaming<Update>>) -> Result<Response<Record>, Status> {
        let storage = self
            .storage
            .as_ref()
            .ok_or_else(|| Status::unavailable("storage not configured"))?;

        let mut stream = request.into_inner();
        let mut updates = Vec::new();

        while let Some(result) = stream.next().await {
            match result {
                Ok(update) => {
                    if let Some(converted) = translate::reverse::update(update) {
                        updates.push(converted);
                    }
                }
                Err(status) => return Err(status),
            }
        }

        let sequence = self.sequence.fetch_add(1, Ordering::Relaxed);
        let path = storage.join(format!("{sequence}.json"));

        let result = record::write(&path, &updates)
            .map_err(|e| Status::internal(format!("failed to write recording: {e}")))?;

        Ok(Response::new(Record {
            path: result.path,
            count: result.count,
            duration: result.duration,
        }))
    }
}

#[tonic::async_trait]
impl Source for Portal {
    type EmitStream = Emission;
    type ReplayStream = Emission;

    async fn emit(
        &self,
        _request: Request<Streaming<Command>>,
    ) -> Result<Response<Self::EmitStream>, Status> {
        let receiver = self.sender.subscribe();
        let output = BroadcastStream::new(receiver).filter_map(std::result::Result::ok);

        Ok(Response::new(Box::pin(output.map(Ok))))
    }

    async fn replay(
        &self,
        request: Request<Record>,
    ) -> Result<Response<Self::ReplayStream>, Status> {
        let record = request.into_inner();

        let updates = record::read(&record.path)
            .map_err(|e| Status::not_found(format!("failed to read recording: {e}")))?;

        let converted = updates
            .into_iter()
            .map(|u| Ok(convert(u)))
            .collect::<Vec<_>>();

        let stream = tokio_stream::iter(converted);

        Ok(Response::new(Box::pin(stream)))
    }
}

fn convert(update: stream::Update) -> Update {
    match update {
        stream::Update::Span(s) => Update {
            payload: Some(proto::observation::update::Payload::Span(translate::span(
                s,
            ))),
        },
        stream::Update::Event(e) => Update {
            payload: Some(proto::observation::update::Payload::Event(
                translate::event(e),
            )),
        },
        stream::Update::Snapshot(s) => Update {
            payload: Some(proto::observation::update::Payload::Snapshot(
                translate::snapshot(s),
            )),
        },
    }
}

pub async fn serve(address: SocketAddr, portal: Arc<Portal>) -> error::Result<()> {
    tonic::transport::Server::builder()
        .add_service(SinkServer::from_arc(portal.clone()))
        .add_service(SourceServer::from_arc(portal))
        .serve(address)
        .await
        .map_err(|source| error::Error::Server { source })
}
