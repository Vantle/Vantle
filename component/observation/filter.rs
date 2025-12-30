use channel::Channel;
use std::sync::Arc;
use tracing::span::Attributes;
use tracing::Subscriber;
use tracing_subscriber::layer::Context;
use tracing_subscriber::registry::LookupSpan;
use tracing_subscriber::Layer;

pub type Predicate = Arc<dyn Fn(&[Channel]) -> bool + Send + Sync>;

pub struct Filter {
    predicate: Predicate,
}

impl Filter {
    #[must_use]
    pub fn new<F>(predicate: F) -> Self
    where
        F: Fn(&[Channel]) -> bool + Send + Sync + 'static,
    {
        Self {
            predicate: Arc::new(predicate),
        }
    }
}

struct Dominated(bool);

#[derive(Default)]
struct Extractor(String);

impl tracing::field::Visit for Extractor {
    fn record_str(&mut self, field: &tracing::field::Field, value: &str) {
        if field.name() == "channels" {
            self.0 = value.to_string();
        }
    }

    fn record_debug(&mut self, _: &tracing::field::Field, _: &dyn std::fmt::Debug) {}
}

impl<S> Layer<S> for Filter
where
    S: Subscriber + for<'lookup> LookupSpan<'lookup>,
{
    fn on_new_span(&self, attrs: &Attributes<'_>, id: &tracing::span::Id, ctx: Context<'_, S>) {
        let mut extractor = Extractor::default();
        attrs.record(&mut extractor);

        let channels = match Channel::parse(&extractor.0) {
            Ok(parsed) => parsed,
            Err(error) => {
                tracing::error!(?error, "failed to parse channel specification");
                return;
            }
        };
        let dominated = (self.predicate)(&channels);

        if let Some(span) = ctx.span(id) {
            span.extensions_mut().insert(Dominated(dominated));
        }
    }

    fn event_enabled(&self, event: &tracing::Event<'_>, ctx: Context<'_, S>) -> bool {
        ctx.event_span(event)
            .and_then(|span| span.extensions().get::<Dominated>().map(|d| d.0))
            .unwrap_or(false)
    }
}
