use layer::Streamer;
use observation::observe::trace;
use stream::{Lifecycle, Updates};

#[trace(channels = [test])]
fn outer(depth: usize) -> usize {
    if depth > 0 { inner(depth - 1) } else { depth }
}

#[trace(channels = [test])]
fn inner(depth: usize) -> usize {
    if depth > 0 { outer(depth - 1) } else { depth }
}

fn nested(depth: usize) -> (usize, usize) {
    let sink = Streamer::assembler(stream::predicate("test")).open();
    let _ = outer(depth);
    let captured = sink.close().spans().collect::<Vec<_>>();

    let begins = captured
        .iter()
        .filter(|s| matches!(s.lifecycle, Lifecycle::Begin(_)))
        .count();
    let parents = captured
        .iter()
        .filter(|s| matches!(s.lifecycle, Lifecycle::Begin(_)))
        .filter(|s| s.id.parent.is_some())
        .count();

    (begins, parents)
}

fn ancestry(depth: usize) -> Vec<bool> {
    let sink = Streamer::assembler(stream::predicate("test")).open();
    let _ = outer(depth);
    let captured = sink.close().spans().collect::<Vec<_>>();

    captured
        .iter()
        .filter(|s| matches!(s.lifecycle, Lifecycle::Begin(_)))
        .enumerate()
        .map(|(i, span)| {
            if i == 0 {
                span.id.parent.is_none()
            } else {
                span.id.parent.is_some()
            }
        })
        .collect()
}

fn consistent(depth: usize) -> bool {
    let sink = Streamer::assembler(stream::predicate("test")).open();
    let _ = outer(depth);
    let captured = sink.close().spans().collect::<Vec<_>>();

    let begins: Vec<_> = captured
        .iter()
        .filter(|s| matches!(s.lifecycle, Lifecycle::Begin(_)))
        .collect();

    if begins.is_empty() {
        return true;
    }

    let trace = begins[0].id.trace;
    begins.iter().all(|s| s.id.trace == trace && s.id.span != 0)
}
