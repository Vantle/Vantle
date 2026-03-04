use component::arena::Valued;
use layer::Streamer;
use stream::{Event, Updates};
use system::arena::{Aliased, Allocatable, Indexed};

fn capture<F: FnOnce() -> R, R>(emit: F) -> (R, Vec<Event>) {
    let sink = Streamer::assembler(stream::predicate("arena")).open();
    let result = emit();
    let events = sink.close().events().collect::<Vec<_>>();
    (result, events)
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
    let captured = event
        .and_then(|e| stream::field(&e.fields, "id"))
        .map(ToString::to_string);
    let state = event
        .and_then(|e| stream::field(&e.fields, "state"))
        .map(ToString::to_string);
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
    let captured = event
        .and_then(|e| stream::field(&e.fields, "id"))
        .map(ToString::to_string);
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
    let captured = event
        .and_then(|e| stream::field(&e.fields, "id"))
        .map(ToString::to_string);
    (chs, captured, found)
}
