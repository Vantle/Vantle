use assemble::Assemble;
use collector::tracing_subscriber::Registry;
use collector::tracing_subscriber::layer::SubscriberExt;
use layer::Streamer;
use observation::record;
use stream::{Event, Update};

fn capture<F: FnOnce()>(emit: F) -> Vec<Event> {
    let (streamer, mut receiver) =
        Streamer::assembler(|channels| channels.iter().any(|c| c.name == "test")).assemble();
    let subscriber = Registry::default().with(streamer);
    collector::tracing::subscriber::with_default(subscriber, || {
        let _guard = tracing::debug_span!("test", channels = "test:1").entered();
        emit();
    });
    let mut events = Vec::new();
    while let Ok(update) = receiver.try_recv() {
        if let Update::Event(event) = update {
            events.push(event);
        }
    }
    events
}

fn leveled(level: usize) -> (usize, String) {
    let events = capture(|| match level {
        0 => record::trace!(level = 0),
        1 => record::debug!(level = 1),
        2 => record::info!(level = 2),
        3 => record::warn!(level = 3),
        4 => record::error!(level = 4),
        _ => {}
    });
    let captured = events
        .first()
        .map(|e| format!("{:?}", e.metadata.level))
        .unwrap_or_default();
    (events.len(), captured)
}
