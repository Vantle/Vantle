use assemble::Assemble;
use collector::tracing_subscriber::Registry;
use collector::tracing_subscriber::layer::SubscriberExt;
use layer::Streamer;
use observation::observe::trace;
use observation::record;
use stream::{Event, Field, Update, Value};

fn capture<F: FnOnce()>(emit: F) -> Vec<Event> {
    let (streamer, mut receiver) =
        Streamer::assembler(|channels| channels.iter().any(|c| c.name == "test")).assemble();
    let subscriber = Registry::default().with(streamer);
    collector::tracing::subscriber::with_default(subscriber, emit);
    let mut events = Vec::new();
    while let Ok(update) = receiver.try_recv() {
        if let Update::Event(event) = update {
            events.push(event);
        }
    }
    events
}

fn field(fields: &[Field], name: &str) -> Option<String> {
    fields
        .iter()
        .find(|f| f.name == name)
        .map(|f| match &f.value {
            Value::Signed(v) => v.to_string(),
            Value::Unsigned(v) => v.to_string(),
            Value::Boolean(v) => v.to_string(),
            Value::Text(v) => v.clone(),
            Value::Serialized(v) => format!("{v:?}"),
        })
}

#[trace(channels = [test])]
fn traced(value: usize) -> usize {
    record::info!(inside = value);
    value * 2
}

fn instrumented(value: usize) -> (usize, String, String) {
    let events = capture(|| {
        let _ = traced(value);
    });
    let event = events.first();
    let level = event
        .map(|e| format!("{:?}", e.metadata.level))
        .unwrap_or_default();
    let captured = event
        .and_then(|e| field(&e.fields, "inside"))
        .unwrap_or_default();
    (events.len(), level, captured)
}
