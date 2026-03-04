use layer::Streamer;
use observation::observe::trace;
use observation::record;
use stream::Updates;

#[trace(channels = [test])]
fn traced(value: usize) -> usize {
    record::info!(inside = value);
    value * 2
}

fn instrumented(value: usize) -> (usize, String, String) {
    let sink = Streamer::assembler(stream::predicate("test")).open();
    let _ = traced(value);
    let collected = sink.close().events().collect::<Vec<_>>();
    let event = collected.first();
    let level = event
        .map(|e| format!("{:?}", e.metadata.level))
        .unwrap_or_default();
    let captured = event
        .and_then(|e| stream::field(&e.fields, "inside"))
        .map(ToString::to_string)
        .unwrap_or_default();
    (collected.len(), level, captured)
}
