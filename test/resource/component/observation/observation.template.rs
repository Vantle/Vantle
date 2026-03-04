use layer::Streamer;
use observation::observe::trace;
use observation::record;
use stream::Updates;

#[trace(channels = [test])]
fn compute(value: usize) -> usize {
    record::info!(result = value * 2);
    value * 2
}

fn traced(value: usize) -> (usize, String, String) {
    let sink = Streamer::assembler(stream::predicate("test")).open();
    let result = compute(value);
    let event = sink.close().events().next();
    let level = event
        .as_ref()
        .map(|e| format!("{:?}", e.metadata.level))
        .unwrap_or_default();
    let captured = event
        .as_ref()
        .and_then(|e| stream::field(&e.fields, "result"))
        .map(ToString::to_string)
        .unwrap_or_default();
    (result, level, captured)
}

fn identity(value: usize) -> usize {
    value
}
