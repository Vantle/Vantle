use assemble::Assemble;
use collector::tracing_subscriber::Registry;
use collector::tracing_subscriber::layer::SubscriberExt;
use layer::Streamer;
use observation::observe::trace;
use stream::{Lifecycle, Span, Update};

fn spans<F: FnOnce()>(emit: F) -> Vec<Span> {
    let (streamer, mut receiver) =
        Streamer::assembler(|channels| channels.iter().any(|c| c.name == "test")).assemble();
    let subscriber = Registry::default().with(streamer);
    collector::tracing::subscriber::with_default(subscriber, emit);
    let mut captured = Vec::new();
    while let Ok(update) = receiver.try_recv() {
        if let Update::Span(span) = update {
            captured.push(span);
        }
    }
    captured
}

#[trace(channels = [test])]
fn outer(depth: usize) -> usize {
    if depth > 0 { inner(depth - 1) } else { depth }
}

#[trace(channels = [test])]
fn inner(depth: usize) -> usize {
    if depth > 0 { outer(depth - 1) } else { depth }
}

fn nested(depth: usize) -> (usize, usize) {
    let captured = spans(|| {
        let _ = outer(depth);
    });

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
    let captured = spans(|| {
        let _ = outer(depth);
    });

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
    let captured = spans(|| {
        let _ = outer(depth);
    });

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
