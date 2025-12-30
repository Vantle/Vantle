use channel::Channel;
use dashmap::DashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, OnceLock};
use tokio::sync::mpsc;
use tracing::span::Attributes;
use tracing::Subscriber;
use tracing_subscriber::layer::Context;
use tracing_subscriber::registry::LookupSpan;
use tracing_subscriber::Layer;

use stream::{
    generate, Begin, End, Event, Identifier, Level, Lifecycle, Metadata, Span, Update, Visitor,
};

struct State {
    id: Identifier,
    metadata: Metadata,
    channels: Vec<Channel>,
}

pub type Sender = mpsc::Sender<Update>;
pub type Receiver = mpsc::Receiver<Update>;
pub type Predicate = Arc<dyn Fn(&[Channel]) -> bool + Send + Sync>;

pub struct Assembler<F> {
    predicate: F,
    capacity: usize,
}

impl<F> Assembler<F>
where
    F: Fn(&[Channel]) -> bool + Send + Sync + 'static,
{
    #[must_use]
    pub fn capacity(mut self, capacity: usize) -> Self {
        self.capacity = capacity;
        self
    }

    #[must_use]
    pub fn assemble(self) -> (Streamer, Receiver) {
        let (sender, receiver) = mpsc::channel(self.capacity);
        let streamer = Streamer {
            sender,
            predicate: Arc::new(self.predicate),
            spans: DashMap::new(),
            trace: OnceLock::new(),
            dropped: AtomicU64::new(0),
        };
        streamer.trace.get_or_init(generate);
        (streamer, receiver)
    }
}

pub struct Streamer {
    sender: Sender,
    predicate: Predicate,
    spans: DashMap<tracing::span::Id, State>,
    trace: OnceLock<u64>,
    dropped: AtomicU64,
}

impl Streamer {
    #[must_use]
    pub fn assembler<F>(predicate: F) -> Assembler<F>
    where
        F: Fn(&[Channel]) -> bool + Send + Sync + 'static,
    {
        Assembler {
            predicate,
            capacity: 65536,
        }
    }

    fn emit(&self, update: Update) {
        if self.sender.try_send(update).is_err() {
            self.dropped.fetch_add(1, Ordering::Relaxed);
        }
    }

    fn identifier(&self, id: &tracing::span::Id, parent: Option<&tracing::span::Id>) -> Identifier {
        let trace = self.trace.get().copied().unwrap_or_else(generate);
        let span = id.into_u64();
        match parent.and_then(|p| self.spans.get(p)) {
            Some(state) => Identifier::child(trace, span, state.id.span),
            None => Identifier::root(trace, span),
        }
    }

    #[must_use]
    pub fn dropped(&self) -> u64 {
        self.dropped.load(Ordering::Relaxed)
    }
}

#[derive(Default)]
struct Extractor {
    channels: String,
    fields: Vec<stream::Field>,
}

impl tracing::field::Visit for Extractor {
    fn record_str(&mut self, field: &tracing::field::Field, value: &str) {
        if field.name() == "channels" {
            self.channels = value.to_string();
        } else {
            self.fields.push(stream::Field {
                name: field.name().to_string(),
                value: stream::Value::Text(value.to_string()),
            });
        }
    }

    fn record_debug(&mut self, field: &tracing::field::Field, value: &dyn std::fmt::Debug) {
        self.fields.push(stream::Field {
            name: field.name().to_string(),
            value: stream::Value::Text(format!("{value:?}")),
        });
    }

    fn record_i64(&mut self, field: &tracing::field::Field, value: i64) {
        self.fields.push(stream::Field {
            name: field.name().to_string(),
            value: stream::Value::Signed(value),
        });
    }

    fn record_u64(&mut self, field: &tracing::field::Field, value: u64) {
        self.fields.push(stream::Field {
            name: field.name().to_string(),
            value: stream::Value::Unsigned(value),
        });
    }

    fn record_bool(&mut self, field: &tracing::field::Field, value: bool) {
        self.fields.push(stream::Field {
            name: field.name().to_string(),
            value: stream::Value::Boolean(value),
        });
    }
}

impl<S> Layer<S> for Streamer
where
    S: Subscriber + for<'lookup> LookupSpan<'lookup>,
{
    fn on_new_span(&self, attrs: &Attributes<'_>, id: &tracing::span::Id, ctx: Context<'_, S>) {
        let meta = attrs.metadata();

        let mut extractor = Extractor::default();
        attrs.record(&mut extractor);

        let channels = match Channel::parse(&extractor.channels) {
            Ok(parsed) => parsed,
            Err(error) => {
                tracing::error!(?error, "failed to parse channel specification");
                return;
            }
        };

        if !(self.predicate)(&channels) {
            return;
        }

        let parent = ctx
            .span_scope(id)
            .and_then(|mut scope| scope.nth(1).map(|span| span.id()));

        let identifier = self.identifier(id, parent.as_ref());
        let metadata = Metadata {
            target: meta.target().to_string(),
            name: meta.name().to_string(),
            level: Level::from(*meta.level()),
        };

        self.spans.insert(
            id.clone(),
            State {
                id: identifier,
                metadata: metadata.clone(),
                channels: channels.clone(),
            },
        );

        let span = Span {
            id: identifier,
            metadata,
            channels,
            lifecycle: Lifecycle::Begin(Begin::now(extractor.fields)),
        };

        self.emit(Update::Span(span));
    }

    fn on_close(&self, id: tracing::span::Id, _ctx: Context<'_, S>) {
        if let Some((_, state)) = self.spans.remove(&id) {
            let span = Span {
                id: state.id,
                metadata: state.metadata,
                channels: state.channels,
                lifecycle: Lifecycle::End(End::now()),
            };
            self.emit(Update::Span(span));
        }
    }

    fn on_event(&self, event: &tracing::Event<'_>, ctx: Context<'_, S>) {
        let dominated = ctx
            .event_span(event)
            .is_some_and(|span| self.spans.contains_key(&span.id()));

        if !dominated {
            return;
        }

        let meta = event.metadata();

        let parent = ctx
            .event_span(event)
            .map(|span| self.spans.get(&span.id()).map_or(0, |state| state.id.span));

        let mut visitor = Visitor::default();
        event.record(&mut visitor);

        let observation = Event::now(
            parent,
            Metadata {
                target: meta.target().to_string(),
                name: meta.name().to_string(),
                level: Level::from(*meta.level()),
            },
            visitor.fields,
        );

        self.emit(Update::Event(observation));
    }
}

pub fn snapshot(sender: &Sender, state: Vec<u8>, trigger: String) {
    let snapshot = stream::Snapshot::now(state, trigger);
    let _ = sender.try_send(Update::Snapshot(snapshot));
}
