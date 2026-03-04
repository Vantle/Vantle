use tracing::Subscriber;
use tracing::span::Attributes;
use tracing_subscriber::Layer;
use tracing_subscriber::layer::Context;
use tracing_subscriber::registry::LookupSpan;

use stream::Predicate;
use visitor::Visitor;

pub struct Filter {
    predicate: Predicate,
}

impl Filter {
    #[must_use]
    pub fn new(predicate: Predicate) -> Self {
        Self { predicate }
    }
}

struct Dominated(bool);

impl<S> Layer<S> for Filter
where
    S: Subscriber + for<'lookup> LookupSpan<'lookup>,
{
    fn on_new_span(&self, attrs: &Attributes<'_>, id: &tracing::span::Id, ctx: Context<'_, S>) {
        let mut visitor = Visitor::default();
        attrs.record(&mut visitor);

        let dominated = match visitor.channels {
            None => true,
            Some(_) => match visitor.channels() {
                Ok(c) => (self.predicate)(&c),
                Err(e) => {
                    tracing::warn!("{:?}", miette::Report::new(e));
                    return;
                }
            },
        };

        if let Some(span) = ctx.span(id) {
            span.extensions_mut().insert(Dominated(dominated));
        }
    }

    fn event_enabled(&self, event: &tracing::Event<'_>, ctx: Context<'_, S>) -> bool {
        ctx.event_span(event)
            .and_then(|span| span.extensions().get::<Dominated>().map(|d| d.0))
            .unwrap_or(true)
    }
}
