use layer::Streamer;
use observation::record;
use stream::Updates;

fn leveled(level: usize) -> (usize, String) {
    let sink = Streamer::assembler(stream::predicate("test")).open();
    let guard = tracing::debug_span!("test", channels = "test:1").entered();
    match level {
        0 => record::trace!(level = 0),
        1 => record::debug!(level = 1),
        2 => record::info!(level = 2),
        3 => record::warn!(level = 3),
        4 => record::error!(level = 4),
        _ => {}
    }
    drop(guard);
    let captured = sink
        .close()
        .events()
        .next()
        .map(|e| format!("{:?}", e.metadata.level))
        .unwrap_or_default();
    (usize::from(!captured.is_empty()), captured)
}
