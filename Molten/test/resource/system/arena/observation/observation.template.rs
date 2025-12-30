use assemble::Assemble;
use collector::tracing_subscriber::Registry;
use collector::tracing_subscriber::layer::SubscriberExt;
use component::arena::Valued;
use layer::Streamer;
use stream::{Event, Field, Update, Value};
use system::arena::{Aliased, Allocatable, Indexed};

fn capture<F: FnOnce() -> R, R>(emit: F) -> (R, Vec<Event>) {
    let (streamer, mut receiver) =
        Streamer::assembler(|channels| channels.iter().any(|c| c.name == "arena")).assemble();
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
            Value::Serialized(v) => String::from_utf8_lossy(v).to_string(),
        })
}

fn channels(event: &Event) -> Vec<String> {
    event.channels.iter().map(|c| c.name.clone()).collect()
}

fn allocate(existing: bool) -> (usize, Vec<String>, Option<String>, Option<String>) {
    let mut arena = Valued::<usize>::default();
    let element = 42usize;

    if existing {
        let _ = arena.allocate(element);
    }

    let (result, events) = capture(|| arena.allocate(element));
    let id = result.expect("allocation failed");

    let event = events.first();
    let chs = event.map(channels).unwrap_or_default();
    let captured = event.and_then(|e| field(&e.fields, "id"));
    let state = event.and_then(|e| field(&e.fields, "state"));
    (id, chs, captured, state)
}

fn value(valid: bool) -> (Vec<String>, Option<String>, bool) {
    let mut arena = Valued::<usize>::default();
    let element = 100usize;
    let id = arena.allocate(element).expect("allocation failed");

    let lookup = if valid { id } else { id + 999 };

    let (result, events) = capture(|| arena.value(lookup));
    let found = result.is_ok();

    let event = events.first();
    let chs = event.map(channels).unwrap_or_default();
    let captured = event.and_then(|e| field(&e.fields, "id"));
    (chs, captured, found)
}

fn alias(valid: bool) -> (Vec<String>, Option<String>, bool) {
    let mut arena = Valued::<usize>::default();
    let element = 200usize;
    let _ = arena.allocate(element).expect("allocation failed");

    let lookup = if valid { element } else { 999 };

    let (result, events) = capture(|| arena.alias(&lookup));
    let found = result.is_ok();

    let event = events.first();
    let chs = event.map(channels).unwrap_or_default();
    let captured = event.and_then(|e| field(&e.fields, "id"));
    (chs, captured, found)
}
