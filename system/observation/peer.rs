pub use error;

use assemble::Assemble;
use dashmap::DashMap;
use proto::observation::sink_client::SinkClient;
use proto::observation::sink_server::{Sink, SinkServer};
use proto::observation::source_client::SourceClient;
use proto::observation::source_server::{Source, SourceServer};
use proto::observation::{Acknowledge, Command, Record, Update};
use std::net::SocketAddr;
use std::pin::Pin;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use tokio::sync::broadcast;
use tokio::sync::mpsc::{self, Receiver, Sender};
use tokio_stream::{Stream, StreamExt};
use tokio_util::sync::CancellationToken;
use tonic::{Request, Response, Status, Streaming};
use url::Url;

type Emission = Pin<Box<dyn Stream<Item = Result<Update, Status>> + Send>>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Role {
    Sink,
    Source,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Handle(u64);

#[derive(Debug, Clone)]
pub struct Descriptor {
    pub handle: Handle,
    pub address: Url,
    pub role: Role,
}

struct Connection {
    address: Url,
    role: Role,
    shutdown: CancellationToken,
}

pub struct Assembler {
    address: Option<Url>,
    sinks: Vec<Url>,
    sources: Vec<Url>,
    capacity: usize,
}

impl Assembler {
    #[must_use]
    pub fn new() -> Self {
        Self {
            address: None,
            sinks: Vec::new(),
            sources: Vec::new(),
            capacity: 65536,
        }
    }

    #[must_use]
    pub fn address(mut self, url: Url) -> Self {
        self.address = Some(url);
        self
    }

    #[must_use]
    pub fn sink(mut self, url: Url) -> Self {
        self.sinks.push(url);
        self
    }

    #[must_use]
    pub fn source(mut self, url: Url) -> Self {
        self.sources.push(url);
        self
    }

    #[must_use]
    pub fn capacity(mut self, capacity: usize) -> Self {
        self.capacity = capacity;
        self
    }
}

impl Assemble for Assembler {
    type Output = Peer;

    fn assemble(self) -> Self::Output {
        let address = self
            .address
            .unwrap_or_else(|| Url::parse("grpc://127.0.0.1:0").expect("valid default URL"));

        let (outgoing, _) = broadcast::channel::<stream::Update>(self.capacity);
        let (incoming, receiver) = mpsc::channel::<stream::Update>(self.capacity);

        let peer = Peer {
            address,
            connections: DashMap::new(),
            sequence: AtomicU64::new(0),
            outgoing: Arc::new(outgoing),
            incoming,
            receiver: tokio::sync::Mutex::new(receiver),
        };

        for url in self.sinks {
            let _ = peer.sink(url);
        }

        for url in self.sources {
            let _ = peer.source(url);
        }

        peer
    }
}

impl Default for Assembler {
    fn default() -> Self {
        Self::new()
    }
}

pub struct Peer {
    address: Url,
    connections: DashMap<Handle, Connection>,
    sequence: AtomicU64,
    outgoing: Arc<broadcast::Sender<stream::Update>>,
    incoming: Sender<stream::Update>,
    receiver: tokio::sync::Mutex<Receiver<stream::Update>>,
}

impl Peer {
    #[must_use]
    pub fn address(&self) -> &Url {
        &self.address
    }

    pub fn sink(&self, url: Url) -> error::Result<Handle> {
        let handle = Handle(self.sequence.fetch_add(1, Ordering::Relaxed));
        let shutdown = CancellationToken::new();

        let connection = Connection {
            address: url.clone(),
            role: Role::Sink,
            shutdown: shutdown.clone(),
        };

        self.connections.insert(handle, connection);

        let outgoing = self.outgoing.clone();
        let endpoint = endpoint(&url);

        tokio::spawn(async move {
            let Ok(mut client) = SinkClient::connect(endpoint).await else {
                return;
            };

            let mut subscription = outgoing.subscribe();
            let stream = async_stream::stream! {
                loop {
                    tokio::select! {
                        () = shutdown.cancelled() => break,
                        received = subscription.recv() => {
                            match received {
                                Ok(update) => yield encode::proto::update(update),
                                Err(_) => break,
                            }
                        }
                    }
                }
            };

            let _ = client.send(stream).await;
        });

        Ok(handle)
    }

    pub fn source(&self, url: Url) -> error::Result<Handle> {
        let handle = Handle(self.sequence.fetch_add(1, Ordering::Relaxed));
        let shutdown = CancellationToken::new();

        let connection = Connection {
            address: url.clone(),
            role: Role::Source,
            shutdown: shutdown.clone(),
        };

        self.connections.insert(handle, connection);

        let incoming = self.incoming.clone();
        let endpoint = endpoint(&url);

        tokio::spawn(async move {
            let Ok(mut client) = SourceClient::connect(endpoint).await else {
                return;
            };

            let commands = tokio_stream::empty::<Command>();
            let Ok(response) = client.emit(commands).await else {
                return;
            };

            let mut stream = response.into_inner();
            loop {
                tokio::select! {
                    () = shutdown.cancelled() => break,
                    received = stream.next() => {
                        match received {
                            Some(Ok(update)) => {
                                if let Some(converted) = decode::proto::update(update) {
                                    let _ = incoming.send(converted).await;
                                }
                            }
                            _ => break,
                        }
                    }
                }
            }
        });

        Ok(handle)
    }

    pub fn disconnect(&self, handle: Handle) -> error::Result<()> {
        match self.connections.remove(&handle) {
            Some((_, connection)) => {
                connection.shutdown.cancel();
                Ok(())
            }
            None => Err(error::Error::Handle { handle: handle.0 }),
        }
    }

    pub fn send(&self, update: stream::Update) -> error::Result<()> {
        self.outgoing
            .send(update)
            .map_err(|_| error::Error::Connection {
                details: "no receivers".to_string(),
            })?;
        Ok(())
    }

    pub async fn next(&self) -> Option<stream::Update> {
        self.receiver.lock().await.recv().await
    }

    #[must_use]
    pub fn connections(&self) -> Vec<Descriptor> {
        self.connections
            .iter()
            .map(|entry| Descriptor {
                handle: *entry.key(),
                address: entry.value().address.clone(),
                role: entry.value().role,
            })
            .collect::<Vec<_>>()
    }

    pub async fn serve(&self) -> error::Result<()> {
        let address = socket(&self.address)?;

        let collector = Collector::new(self.incoming.clone());
        let emitter = Emitter::new(self.outgoing.clone());

        tonic::transport::Server::builder()
            .add_service(SinkServer::new(collector))
            .add_service(SourceServer::new(emitter))
            .serve(address)
            .await
            .map_err(|source| error::Error::Server { source })
    }
}

struct Collector {
    sender: Sender<stream::Update>,
}

impl Collector {
    fn new(sender: Sender<stream::Update>) -> Self {
        Self { sender }
    }
}

#[tonic::async_trait]
impl Sink for Collector {
    async fn send(
        &self,
        request: Request<Streaming<Update>>,
    ) -> Result<Response<Acknowledge>, Status> {
        let mut stream = request.into_inner();

        while let Some(result) = stream.next().await {
            match result {
                Ok(update) => {
                    if let Some(converted) = decode::proto::update(update) {
                        let _ = self.sender.send(converted).await;
                    }
                }
                Err(status) => return Err(status),
            }
        }

        Ok(Response::new(Acknowledge {}))
    }

    async fn store(
        &self,
        _request: Request<Streaming<Update>>,
    ) -> Result<Response<Record>, Status> {
        Err(Status::unimplemented("store not supported"))
    }
}

struct Emitter {
    broadcast: Arc<broadcast::Sender<stream::Update>>,
}

impl Emitter {
    fn new(broadcast: Arc<broadcast::Sender<stream::Update>>) -> Self {
        Self { broadcast }
    }
}

#[tonic::async_trait]
impl Source for Emitter {
    type EmitStream = Emission;
    type ReplayStream = Emission;

    async fn emit(
        &self,
        _request: Request<Streaming<Command>>,
    ) -> Result<Response<Self::EmitStream>, Status> {
        let receiver = self.broadcast.subscribe();
        let stream = tokio_stream::wrappers::BroadcastStream::new(receiver)
            .filter_map(Result::ok)
            .map(|update| Ok(encode::proto::update(update)));

        Ok(Response::new(Box::pin(stream)))
    }

    async fn replay(
        &self,
        _request: Request<Record>,
    ) -> Result<Response<Self::ReplayStream>, Status> {
        Err(Status::unimplemented("replay not supported"))
    }
}

fn endpoint(url: &Url) -> String {
    let host = url.host_str().unwrap_or("127.0.0.1");
    let port = url.port().unwrap_or(50051);
    format!("http://{host}:{port}")
}

fn socket(url: &Url) -> error::Result<SocketAddr> {
    let host = url.host_str().unwrap_or("127.0.0.1");
    let port = url.port().unwrap_or(0);
    let address = format!("{host}:{port}");

    address
        .parse()
        .map_err(|source| error::Error::Address { address, source })
}
