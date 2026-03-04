pub use channel;
pub use error;
pub use grpc::Status;

use endpoint::{Sink, Stream};
use expression::Expression;
use filter::Filter;
use std::fs::File;
use std::io::LineWriter;
use std::sync::Mutex;
use tokio_util::sync::CancellationToken;
use tracing_chrome::{ChromeLayer, FlushGuard};
use tracing_subscriber::prelude::*;
use tracing_subscriber::{Registry, fmt};

enum Output {
    Log(
        fmt::Layer<
            Registry,
            fmt::format::DefaultFields,
            fmt::format::Format,
            Mutex<LineWriter<File>>,
        >,
    ),
    Chrome(ChromeLayer<Registry>),
    Grpc(layer::Streamer),
}

macro_rules! delegate {
    ($self:ident, $method:ident($($arg:ident),*)) => {
        match $self {
            Self::Log(output) => {
                tracing_subscriber::Layer::<Registry>::$method(output, $($arg),*)
            }
            Self::Chrome(output) => {
                tracing_subscriber::Layer::<Registry>::$method(output, $($arg),*)
            }
            Self::Grpc(output) => {
                tracing_subscriber::Layer::<Registry>::$method(output, $($arg),*)
            }
        }
    };
}

impl tracing_subscriber::Layer<Registry> for Output {
    fn on_register_dispatch(&self, subscriber: &tracing::Dispatch) {
        delegate!(self, on_register_dispatch(subscriber));
    }

    fn on_layer(&mut self, subscriber: &mut Registry) {
        delegate!(self, on_layer(subscriber));
    }

    fn register_callsite(
        &self,
        metadata: &'static tracing::Metadata<'static>,
    ) -> tracing::subscriber::Interest {
        delegate!(self, register_callsite(metadata))
    }

    fn enabled(
        &self,
        metadata: &tracing::Metadata<'_>,
        ctx: tracing_subscriber::layer::Context<'_, Registry>,
    ) -> bool {
        delegate!(self, enabled(metadata, ctx))
    }

    fn on_new_span(
        &self,
        attrs: &tracing::span::Attributes<'_>,
        id: &tracing::span::Id,
        ctx: tracing_subscriber::layer::Context<'_, Registry>,
    ) {
        delegate!(self, on_new_span(attrs, id, ctx));
    }

    fn on_record(
        &self,
        span: &tracing::span::Id,
        values: &tracing::span::Record<'_>,
        ctx: tracing_subscriber::layer::Context<'_, Registry>,
    ) {
        delegate!(self, on_record(span, values, ctx));
    }

    fn on_follows_from(
        &self,
        span: &tracing::span::Id,
        follows: &tracing::span::Id,
        ctx: tracing_subscriber::layer::Context<'_, Registry>,
    ) {
        delegate!(self, on_follows_from(span, follows, ctx));
    }

    fn on_event(
        &self,
        event: &tracing::Event<'_>,
        ctx: tracing_subscriber::layer::Context<'_, Registry>,
    ) {
        delegate!(self, on_event(event, ctx));
    }

    fn on_enter(
        &self,
        id: &tracing::span::Id,
        ctx: tracing_subscriber::layer::Context<'_, Registry>,
    ) {
        delegate!(self, on_enter(id, ctx));
    }

    fn on_exit(
        &self,
        id: &tracing::span::Id,
        ctx: tracing_subscriber::layer::Context<'_, Registry>,
    ) {
        delegate!(self, on_exit(id, ctx));
    }

    fn on_close(
        &self,
        id: tracing::span::Id,
        ctx: tracing_subscriber::layer::Context<'_, Registry>,
    ) {
        delegate!(self, on_close(id, ctx));
    }

    fn on_id_change(
        &self,
        old: &tracing::span::Id,
        new: &tracing::span::Id,
        ctx: tracing_subscriber::layer::Context<'_, Registry>,
    ) {
        delegate!(self, on_id_change(old, new, ctx));
    }
}

pub struct Guard {
    flush: Option<FlushGuard>,
    cancellation: Option<CancellationToken>,
    tasks: Vec<tokio::task::JoinHandle<()>>,
}

impl Guard {
    pub fn track(&mut self, handle: tokio::task::JoinHandle<()>) {
        self.tasks.push(handle);
    }
}

impl Drop for Guard {
    fn drop(&mut self) {
        if let Some(token) = self.cancellation.take() {
            token.cancel();
        }
        if let Ok(runtime) = tokio::runtime::Handle::try_current() {
            for task in self.tasks.drain(..) {
                let _ = runtime.block_on(task);
            }
        }
        self.flush.take();
        grpc::flush();
    }
}

pub fn initialize(sinks: Vec<Sink>, cancellation: CancellationToken) -> error::Result<Guard> {
    let mut guard = Guard {
        flush: None,
        cancellation: Some(cancellation.clone()),
        tasks: Vec::new(),
    };
    let mut expressions = Vec::new();

    let (layers, endpoints) = sinks.iter().try_fold(
        (Vec::new(), Vec::new()),
        |(mut layers, mut endpoints), sink| -> error::Result<_> {
            match sink {
                Sink::Log {
                    stream:
                        Stream {
                            url,
                            level,
                            channels,
                        },
                    ansi,
                } => {
                    expressions.push(channels.clone());
                    let path = std::path::PathBuf::from(url.path());
                    layers.push(Output::Log(log::file(&path, *ansi)?).with_filter(*level));
                }
                Sink::Chrome(Stream {
                    url,
                    level,
                    channels,
                }) => {
                    expressions.push(channels.clone());
                    let path = std::path::PathBuf::from(url.path());
                    let (tracer, flush) = chrome::layer(&path)?;
                    guard.flush = Some(flush);
                    layers.push(Output::Chrome(tracer).with_filter(*level));
                }
                Sink::Grpc(Stream {
                    url,
                    level,
                    channels,
                }) => {
                    expressions.push(channels.clone());
                    let (streamer, receiver) = grpc::layer(channels.clone().predicate());
                    layers.push(Output::Grpc(streamer).with_filter(*level));
                    endpoints.push((url.clone(), receiver));
                }
                Sink::Http(_) => {}
            }
            Ok((layers, endpoints))
        },
    )?;

    let predicate = Expression::combine(expressions).predicate();

    Registry::default()
        .with(layers)
        .with(Filter::new(predicate))
        .try_init()
        .map_err(|e| error::Error::Subscriber {
            details: e.to_string(),
        })?;

    for (url, receiver) in endpoints {
        grpc::spawn(url, receiver, cancellation.clone())?;
    }

    Ok(guard)
}
