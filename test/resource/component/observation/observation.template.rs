use assemble::Assemble;
use collector::tracing_subscriber::Registry;
use collector::tracing_subscriber::layer::SubscriberExt;
use layer::Streamer;
use observation::observe::trace;
use observation::record;
use stream::{Event, Field, Update, Value};

fn capture<F: FnOnce() -> R, R>(emit: F) -> (R, Vec<Event>) {
    let (streamer, mut receiver) =
        Streamer::assembler(|channels| channels.iter().any(|c| c.name == "test")).assemble();
    let subscriber = Registry::default().with(streamer);
    let result = collector::tracing::subscriber::with_default(subscriber, emit);
    let mut events = Vec::new();
    while let Ok(update) = receiver.try_recv() {
        if let Update::Event(event) = update {
            events.push(event);
        }
    }
    (result, events)
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
fn compute(value: usize) -> usize {
    record::info!(result = value * 2);
    value * 2
}

fn traced(value: usize) -> (usize, String, String) {
    let (result, events) = capture(|| compute(value));
    let event = events.first();
    let level = event
        .map(|e| format!("{:?}", e.metadata.level))
        .unwrap_or_default();
    let captured = event
        .and_then(|e| field(&e.fields, "result"))
        .unwrap_or_default();
    (result, level, captured)
}

fn identity(value: usize) -> usize {
    value
}
