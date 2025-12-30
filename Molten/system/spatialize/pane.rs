use std::collections::BTreeSet;
use std::fmt::Display;

use error::Result;
use hypergraph::{Hypergraph, Label};
use particle::Particle;
use serde::Serialize;
use serde::de::DeserializeOwned;
use space::{Connector, Node, Space};
use wave::Wave;

const PRODUCT: &str = "Molten";
const MODULE: &str = "Spatialize";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Pane {
    #[default]
    Relation,
    Inference,
}

impl Pane {
    #[must_use]
    pub fn toggle(&self) -> Self {
        match self {
            Pane::Relation => Pane::Inference,
            Pane::Inference => Pane::Relation,
        }
    }

    #[must_use]
    pub fn label(&self) -> &'static str {
        match self {
            Pane::Relation => "Relation",
            Pane::Inference => "Inference",
        }
    }

    #[must_use]
    pub fn title(&self) -> String {
        format!("{PRODUCT}::{MODULE} · {}", self.label())
    }
}

pub fn extract<T>(graph: &Hypergraph<T>, pane: Pane) -> Result<Space>
where
    T: Clone + Eq + Ord + Serialize + DeserializeOwned,
    Particle<T>: Display,
    Wave<T>: Display,
{
    match pane {
        Pane::Relation => relation(graph),
        Pane::Inference => inference(graph),
    }
}

fn relation<T>(graph: &Hypergraph<T>) -> Result<Space>
where
    T: Clone + Eq + Ord + Serialize + DeserializeOwned,
    Particle<T>: Display,
    Wave<T>: Display,
{
    use std::collections::BTreeMap;

    let mut space = Space::new();
    let mut waves = BTreeMap::<String, Label>::new();
    let mut counter = 0usize;

    for edge in &graph.edges {
        let source = edge.relation.source.to_string();
        let sink = edge.relation.sink.to_string();

        let from = *waves.entry(source.clone()).or_insert_with(|| {
            counter += 1;
            Label(counter)
        });

        let to = *waves.entry(sink.clone()).or_insert_with(|| {
            counter += 1;
            Label(counter)
        });

        space.connect(Connector::new(
            edge.label,
            BTreeSet::from([from]),
            BTreeSet::from([to]),
            palette::EDGE,
            scale::stroke(),
        ))?;
    }

    for (text, label) in waves {
        space.insert(Node::new(label, text, scale::glyph()))?;
    }

    Ok(space)
}

fn inference<T>(graph: &Hypergraph<T>) -> Result<Space>
where
    T: Clone + Eq + Ord + Serialize + DeserializeOwned,
    Particle<T>: Display,
{
    let mut space = Space::new();

    for node in &graph.nodes {
        space.insert(Node::new(
            node.label,
            node.particle.to_string(),
            scale::glyph(),
        ))?;
    }

    for edge in &graph.edges {
        let sources = edge.inference.source.clone();
        let sinks = edge.inference.sink.clone();

        space.connect(Connector::new(
            edge.label,
            sources,
            sinks,
            palette::EDGE,
            scale::stroke(),
        ))?;
    }

    Ok(space)
}

pub fn highlight<T>(space: &mut Space, _graph: &Hypergraph<T>, hovered: Label, _pane: Pane)
where
    T: Clone + Eq + Ord + Serialize + DeserializeOwned,
{
    let related = neighbors(space, hovered);
    space.clear();
    space.highlight(related);
}

fn neighbors(space: &Space, hovered: Label) -> BTreeSet<Label> {
    let mut related = BTreeSet::from([hovered]);

    if space.node(hovered).is_some() {
        for connector in space.edges() {
            if connector.sources().contains(&hovered) || connector.sinks().contains(&hovered) {
                related.insert(connector.label());
            }
        }
        return related;
    }

    related
}
