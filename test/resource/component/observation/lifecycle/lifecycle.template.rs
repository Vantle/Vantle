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
fn instrumented() {}

#[trace(channels = [test])]
fn outer() {
    inner();
}

#[trace(channels = [test])]
fn inner() {}

fn lifecycle(scenario: usize) -> (usize, usize, usize) {
    let captured = spans(|| match scenario {
        0 => instrumented(),
        1 => outer(),
        _ => {}
    });
    let begins = captured
        .iter()
        .filter(|s| matches!(s.lifecycle, Lifecycle::Begin(_)))
        .count();
    let ends = captured
        .iter()
        .filter(|s| matches!(s.lifecycle, Lifecycle::End(_)))
        .count();
    (captured.len(), begins, ends)
}

fn hierarchy(scenario: usize) -> (usize, bool) {
    let captured = spans(|| match scenario {
        0 => instrumented(),
        1 => outer(),
        _ => {}
    });
    let roots = captured
        .iter()
        .filter(|s| matches!(s.lifecycle, Lifecycle::Begin(_)))
        .filter(|s| s.id.parent.is_none())
        .count();
    let consistent = captured
        .iter()
        .filter(|s| matches!(s.lifecycle, Lifecycle::Begin(_)))
        .all(|s| s.id.trace != 0 && s.id.span != 0);
    (roots, consistent)
}

fn identifier(mode: usize) -> (bool, bool, bool) {
    let root = stream::Identifier::root(100, 1);
    let child = stream::Identifier::child(100, 2, 1);
    match mode {
        0 => (root.parent.is_none(), root.trace == 100, root.span == 1),
        1 => (child.parent == Some(1), child.trace == 100, child.span == 2),
        _ => (false, false, false),
    }
}
