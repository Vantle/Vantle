use assemble::Assemble;
use collector::tracing_subscriber::Registry;
use collector::tracing_subscriber::layer::SubscriberExt;
use layer::Streamer;
use observation::observe::trace;
use stream::channel::Channel;

fn count<F, P>(predicate: P, emit: F) -> usize
where
    F: FnOnce(),
    P: Fn(&[Channel]) -> bool + Send + Sync + 'static,
{
    let (streamer, mut receiver) = Streamer::assembler(predicate).assemble();
    let subscriber = Registry::default().with(streamer);
    collector::tracing::subscriber::with_default(subscriber, emit);
    let mut captured = 0;
    while receiver.try_recv().is_ok() {
        captured += 1;
    }
    captured
}

#[trace(channels = [alpha])]
fn alpha() {}

#[trace(channels = [beta])]
fn beta() {}

#[trace(channels = [alpha, beta])]
fn combined() {}

fn included(filter: Vec<String>) -> usize {
    count(
        move |channels| channels.iter().any(|c| filter.contains(&c.name)),
        || {
            alpha();
            beta();
            combined();
        },
    )
}

fn excluded(filter: Vec<String>) -> usize {
    count(
        move |channels| !channels.iter().any(|c| filter.contains(&c.name)),
        || {
            alpha();
            beta();
            combined();
        },
    )
}

fn weighted(threshold: u8) -> usize {
    count(
        move |channels| channels.iter().any(|c| c.weight >= threshold),
        || {
            let _low = collector::tracing::info_span!("low", channels = "test:1").entered();
            let _mid = collector::tracing::info_span!("mid", channels = "test:5").entered();
            let _high = collector::tracing::info_span!("high", channels = "test:10").entered();
        },
    )
}

fn parsed(specification: String) -> (usize, u8) {
    let channels = Channel::parse(&specification).unwrap_or_default();
    let weight = channels.first().map_or(0, |c| c.weight);
    (channels.len(), weight)
}

fn serialized(names: Vec<String>, weights: Vec<u8>) -> String {
    let channels: Vec<_> = names
        .into_iter()
        .zip(weights)
        .map(|(name, weight)| Channel { name, weight })
        .collect();
    Channel::serialize(&channels)
}
