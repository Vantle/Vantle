use dashmap::DashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, OnceLock};
use tokio::sync::mpsc;
use tracing::Subscriber;
use tracing::span::Attributes;
use tracing_subscriber::Layer;
use tracing_subscriber::layer::Context;
use tracing_subscriber::registry::LookupSpan;

use assemble::Assemble;
use channel::Channel;
use stream::{
    Begin, End, Event, Identifier, Level, Lifecycle, Metadata, Predicate, Span, Update, generate,
};
use visitor::Visitor;

const CAPACITY: usize = 65536;

fn metadata(meta: &tracing::Metadata<'_>) -> Metadata {
    Metadata {
        target: meta.target().to_string(),
        name: meta.name().to_string(),
        level: Level::from(*meta.level()),
    }
}

struct State {
    id: Identifier,
    metadata: Metadata,
    channels: Vec<Channel>,
}

pub type Sender = mpsc::Sender<Update>;
pub type Receiver = mpsc::Receiver<Update>;

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
}

impl<F> Assemble for Assembler<F>
where
    F: Fn(&[Channel]) -> bool + Send + Sync + 'static,
{
    type Output = (Streamer, Receiver);

    fn assemble(self) -> Self::Output {
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
            capacity: CAPACITY,
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

impl<S> Layer<S> for Streamer
where
    S: Subscriber + for<'lookup> LookupSpan<'lookup>,
{
    fn on_new_span(&self, attrs: &Attributes<'_>, id: &tracing::span::Id, ctx: Context<'_, S>) {
        let meta = attrs.metadata();

        let mut collector = Visitor::default();
        attrs.record(&mut collector);

        let channels = match collector.channels() {
            Ok(c) => c,
            Err(e) => {
                tracing::warn!("{:?}", miette::Report::new(e));
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
        let metadata = metadata(meta);

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
            lifecycle: Lifecycle::Begin(Begin::now(collector.fields)),
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
        let meta = event.metadata();
        let mut collector = Visitor::default();
        event.record(&mut collector);

        if collector.channels.is_some() {
            let channels = match collector.channels() {
                Ok(c) => c,
                Err(e) => {
                    tracing::warn!("{:?}", miette::Report::new(e));
                    return;
                }
            };
            if !(self.predicate)(&channels) {
                return;
            }

            let parent = ctx
                .event_span(event)
                .map(|span| self.spans.get(&span.id()).map_or(0, |state| state.id.span));

            let observation = Event::now(parent, metadata(meta), channels, collector.fields);
            self.emit(Update::Event(observation));
            return;
        }

        let dominated = ctx
            .event_span(event)
            .is_some_and(|span| self.spans.contains_key(&span.id()));
        if !dominated {
            return;
        }

        let parent = ctx
            .event_span(event)
            .map(|span| self.spans.get(&span.id()).map_or(0, |state| state.id.span));

        let observation = Event::now(parent, metadata(meta), vec![], collector.fields);
        self.emit(Update::Event(observation));
    }
}

pub fn snapshot(sender: &Sender, state: Vec<u8>, trigger: String) {
    let snapshot = stream::Snapshot::now(state, trigger);
    let _ = sender.try_send(Update::Snapshot(snapshot));
}
