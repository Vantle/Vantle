use std::collections::{BTreeMap, BTreeSet};

use assemble::Assemble;
use collector::tracing_subscriber::Registry;
use collector::tracing_subscriber::layer::SubscriberExt;
use component::graph::relation::Edge as Relation;
use component::graph::relation::Related;
use component::graph::state::particle::Particle;
use component::graph::state::wave::Wave;
use component::hypergraph::{Hypergraph, Label, Meta};
use layer::Streamer;
use stream::{Event, Field, Update, Value};
use system::hypergraph::evaluate::Evaluate;

fn empty() -> Hypergraph<usize> {
    Hypergraph {
        meta: Meta {},
        nodes: BTreeSet::new(),
        edges: BTreeSet::new(),
        particles: 0,
        refractions: BTreeMap::new(),
        world: BTreeMap::new(),
        worlds: 0,
        united: BTreeMap::new(),
        future: BTreeMap::new(),
        past: BTreeMap::new(),
    }
}

fn capture<F: FnOnce() -> R, R>(channels: &[&str], emit: F) -> (R, Vec<Event>) {
    let filter = channels
        .iter()
        .map(|s| (*s).to_string())
        .collect::<Vec<_>>();
    let (streamer, mut receiver) =
        Streamer::assembler(move |chs| chs.iter().any(|c| filter.contains(&c.name))).assemble();
    let subscriber = Registry::default().with(streamer);
    let result = collector::tracing::subscriber::with_default(subscriber, emit);
    let mut events = Vec::new();
    while let Ok(update) = receiver.try_recv() {
        if let Update::Event(event) = update {
            events.push(event);
        }
    }
    (result, events)
}

fn field(fields: &[Field], name: &str) -> Option<String> {
    fields
        .iter()
        .find(|f| f.name == name)
        .map(|f| match &f.value {
            Value::Signed(v) => v.to_string(),
            Value::Unsigned(v) => v.to_string(),
            Value::Boolean(v) => v.to_string(),
            Value::Text(v) => v.clone(),
            Value::Serialized(v) => String::from_utf8_lossy(v).to_string(),
        })
}

fn channels(event: &Event) -> Vec<String> {
    event.channels.iter().map(|c| c.name.clone()).collect()
}

fn names(event: &Event) -> Vec<String> {
    event.fields.iter().map(|f| f.name.clone()).collect()
}

fn focus() -> (Vec<String>, Vec<String>, Option<String>) {
    let mut graph = empty();
    let particle = Particle::fundamental(1usize);

    let (label, events) = capture(&["hypergraph"], || graph.focus(particle));

    let event = events.first();
    let chs = event.map(channels).unwrap_or_default();
    let flds = event.map(names).unwrap_or_default();
    let captured = event.and_then(|e| field(&e.fields, "label"));
    let _ = label;
    (chs, flds, captured)
}

fn diffuse(count: usize) -> (Vec<String>, Vec<String>, usize) {
    let mut graph = empty();
    let particles = (0..count).map(Particle::fundamental).collect::<Vec<_>>();
    let wave = Wave::from(particles.as_slice());

    let (labels, events) = capture(&["hypergraph"], || graph.diffuse(wave).collect::<Vec<_>>());

    let event = events.last();
    let chs = event.map(channels).unwrap_or_default();
    let flds = event.map(names).unwrap_or_default();
    (chs, flds, labels.len())
}

fn locate(valid: bool) -> (Vec<String>, Vec<String>, bool) {
    let mut graph = empty();
    let particle = Particle::fundamental(1usize);
    let label = graph.focus(particle);

    let lookup = if valid { label } else { Label(9999) };

    let (result, events) = capture(&["hypergraph"], || graph.locate(lookup));
    let found = result.is_ok();

    let event = events.first();
    let chs = event.map(channels).unwrap_or_default();
    let flds = event.map(names).unwrap_or_default();
    (chs, flds, found)
}

fn unite(same: bool) -> (Vec<String>, Vec<String>, bool) {
    let mut graph = empty();
    let p1 = Particle::fundamental(1usize);
    let p2 = Particle::fundamental(2usize);
    let l1 = graph.focus(p1);
    let l2 = if same { l1 } else { graph.focus(p2) };

    let (result, events) = capture(&["hypergraph"], || graph.unite(l1, l2));
    let _ = result.expect("unite failed");

    let event = events.last();
    let chs = event.map(channels).unwrap_or_default();
    let flds = event.map(names).unwrap_or_default();
    let subset = flds.contains(&"subset".to_string());
    (chs, flds, subset)
}

fn translate(existing: bool) -> (Vec<String>, Vec<String>, Option<String>) {
    let mut graph = empty();
    let particle = Particle::fundamental(1usize);
    let l1 = graph.focus(particle.clone());
    let l2 = graph.focus(particle.clone());

    let source = BTreeSet::from([l1]);
    let destinations = BTreeSet::from([l2]);
    let relation = Relation {
        source: Wave::monochromatic(particle.clone()),
        sink: Wave::monochromatic(particle.clone()),
    };

    if existing {
        let _ = graph.translate(source.clone(), destinations.clone(), relation.clone());
    }

    let (result, events) = capture(&["hypergraph"], || {
        graph.translate(source, destinations, relation)
    });
    let _ = result.expect("translate failed");

    let event = events.last();
    let chs = event.map(channels).unwrap_or_default();
    let flds = event.map(names).unwrap_or_default();
    let state = event.and_then(|e| field(&e.fields, "state"));
    (chs, flds, state)
}

fn infer(populated: bool) -> (Vec<String>, Vec<String>, Option<String>) {
    let mut graph = empty();

    if populated {
        let particle = Particle::fundamental(1usize);
        let _ = graph.focus(particle);
    }

    let rules = if populated {
        let source = Wave::monochromatic(Particle::fundamental(1usize));
        let sink = Wave::monochromatic(Particle::fundamental(2usize));
        let mut adjacency = BTreeMap::new();
        adjacency.insert(source, BTreeSet::from([sink]));
        Related::new(adjacency)
    } else {
        Related::default()
    };

    let (result, events) = capture(&["query"], || graph.infer(rules));
    let _ = result.expect("infer failed");

    let event = events.first();
    let chs = event.map(channels).unwrap_or_default();
    let flds = event.map(names).unwrap_or_default();
    let count = event.and_then(|e| field(&e.fields, "count"));
    (chs, flds, count)
}

fn fixed(populated: bool) -> (Vec<String>, Vec<String>, Option<String>) {
    let mut graph = empty();

    if populated {
        let particle = Particle::fundamental(1usize);
        let _ = graph.focus(particle);
    }

    let rules = if populated {
        let source = Wave::monochromatic(Particle::fundamental(1usize));
        let sink = Wave::monochromatic(Particle::fundamental(2usize));
        let mut adjacency = BTreeMap::new();
        adjacency.insert(source, BTreeSet::from([sink]));
        Related::new(adjacency)
    } else {
        Related::default()
    };

    let (result, events) = capture(&["hypergraph"], || graph.fixed(rules));
    let _ = result.expect("fixed failed");

    let event = events.last();
    let chs = event.map(channels).unwrap_or_default();
    let flds = event.map(names).unwrap_or_default();
    let iterations = event.and_then(|e| field(&e.fields, "iterations"));
    (chs, flds, iterations)
}
