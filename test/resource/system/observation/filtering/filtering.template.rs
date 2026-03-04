use layer::Streamer;
use observation::observe::trace;
use stream::channel::Channel;

#[trace(channels = [alpha])]
fn alpha() {}

#[trace(channels = [beta])]
fn beta() {}

#[trace(channels = [alpha, beta])]
fn combined() {}

fn included(filter: Vec<String>) -> usize {
    let sink = Streamer::assembler(std::sync::Arc::new(move |channels: &[_]| {
        channels.iter().any(|c| filter.contains(&c.name))
    }))
    .open();
    alpha();
    beta();
    combined();
    sink.close().count()
}

fn excluded(filter: Vec<String>) -> usize {
    let sink = Streamer::assembler(std::sync::Arc::new(move |channels: &[_]| {
        !channels.iter().any(|c| filter.contains(&c.name))
    }))
    .open();
    alpha();
    beta();
    combined();
    sink.close().count()
}

fn weighted(threshold: u8) -> usize {
    let sink = Streamer::assembler(std::sync::Arc::new(move |channels: &[_]| {
        channels.iter().any(|c| c.weight >= threshold)
    }))
    .open();
    let low = tracing::info_span!("low", channels = "test:1").entered();
    let mid = tracing::info_span!("mid", channels = "test:5").entered();
    let high = tracing::info_span!("high", channels = "test:10").entered();
    drop(high);
    drop(mid);
    drop(low);
    sink.close().count()
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
