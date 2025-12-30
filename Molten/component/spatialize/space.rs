use std::collections::{BTreeSet, HashMap};

use color::Color;
use error::{Duplicate, Error, Missing};
use hypergraph::Label;

#[derive(Debug, Clone, Default)]
pub struct Space {
    items: Vec<Node>,
    connectors: Vec<Connector>,
    highlighted: BTreeSet<Label>,
    index: Index,
}

#[derive(Debug, Clone, Default)]
struct Index {
    nodes: HashMap<Label, usize>,
    edges: HashMap<Label, usize>,
}

#[derive(Debug, Clone)]
pub struct Node {
    label: Label,
    text: String,
    radius: f32,
}

#[derive(Debug, Clone)]
pub struct Connector {
    label: Label,
    sources: BTreeSet<Label>,
    sinks: BTreeSet<Label>,
    color: Color,
    width: f32,
}

impl Space {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&mut self, node: Node) -> Result<(), Error> {
        let label = node.label;
        if self.index.nodes.contains_key(&label) {
            return Err(Duplicate::Node { label }.into());
        }
        let position = self.items.len();
        self.items.push(node);
        self.index.nodes.insert(label, position);
        Ok(())
    }

    pub fn remove(&mut self, label: Label) -> Result<Node, Error> {
        let position = self
            .index
            .nodes
            .remove(&label)
            .ok_or(Missing::Node { label })?;

        let node = self.items.swap_remove(position);

        if position < self.items.len() {
            let swapped = self.items[position].label;
            self.index.nodes.insert(swapped, position);
        }

        self.highlighted.remove(&label);
        Ok(node)
    }

    pub fn connect(&mut self, connector: Connector) -> Result<(), Error> {
        let label = connector.label;
        if self.index.edges.contains_key(&label) {
            return Err(Duplicate::Connector { label }.into());
        }
        let position = self.connectors.len();
        self.connectors.push(connector);
        self.index.edges.insert(label, position);
        Ok(())
    }

    pub fn disconnect(&mut self, label: Label) -> Result<Connector, Error> {
        let position = self
            .index
            .edges
            .remove(&label)
            .ok_or(Missing::Connector { label })?;

        let connector = self.connectors.swap_remove(position);

        if position < self.connectors.len() {
            let swapped = self.connectors[position].label;
            self.index.edges.insert(swapped, position);
        }

        self.highlighted.remove(&label);
        Ok(connector)
    }

    pub fn nodes(&self) -> impl Iterator<Item = &Node> {
        self.items.iter()
    }

    pub fn edges(&self) -> impl Iterator<Item = &Connector> {
        self.connectors.iter()
    }

    #[must_use]
    pub fn count(&self) -> usize {
        self.items.len()
    }

    #[must_use]
    pub fn node(&self, label: Label) -> Option<&Node> {
        self.index
            .nodes
            .get(&label)
            .and_then(|&i| self.items.get(i))
    }

    #[must_use]
    pub fn node_mut(&mut self, label: Label) -> Option<&mut Node> {
        self.index
            .nodes
            .get(&label)
            .and_then(|&i| self.items.get_mut(i))
    }

    #[must_use]
    pub fn connector(&self, label: Label) -> Option<&Connector> {
        self.index
            .edges
            .get(&label)
            .and_then(|&i| self.connectors.get(i))
    }

    #[must_use]
    pub fn connector_mut(&mut self, label: Label) -> Option<&mut Connector> {
        self.index
            .edges
            .get(&label)
            .and_then(|&i| self.connectors.get_mut(i))
    }

    pub fn highlight(&mut self, labels: impl IntoIterator<Item = Label>) {
        self.highlighted.extend(labels);
    }

    pub fn unhighlight(&mut self, labels: impl IntoIterator<Item = Label>) {
        for label in labels {
            self.highlighted.remove(&label);
        }
    }

    pub fn clear(&mut self) {
        self.highlighted.clear();
    }

    #[must_use]
    pub fn highlighted(&self, label: Label) -> bool {
        self.highlighted.contains(&label)
    }
}

impl Node {
    #[must_use]
    pub fn new(label: Label, text: String, radius: f32) -> Self {
        Self {
            label,
            text,
            radius: radius.abs(),
        }
    }

    #[must_use]
    pub fn label(&self) -> Label {
        self.label
    }

    #[must_use]
    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn rename(&mut self, text: String) {
        self.text = text;
    }

    #[must_use]
    pub fn radius(&self) -> f32 {
        self.radius
    }

    pub fn resize(&mut self, radius: f32) {
        self.radius = radius.abs();
    }
}

impl Connector {
    #[must_use]
    pub fn new(
        label: Label,
        sources: BTreeSet<Label>,
        sinks: BTreeSet<Label>,
        color: Color,
        width: f32,
    ) -> Self {
        Self {
            label,
            sources,
            sinks,
            color,
            width: width.abs(),
        }
    }

    #[must_use]
    pub fn label(&self) -> Label {
        self.label
    }

    #[must_use]
    pub fn sources(&self) -> &BTreeSet<Label> {
        &self.sources
    }

    pub fn sources_mut(&mut self) -> &mut BTreeSet<Label> {
        &mut self.sources
    }

    #[must_use]
    pub fn sinks(&self) -> &BTreeSet<Label> {
        &self.sinks
    }

    pub fn sinks_mut(&mut self) -> &mut BTreeSet<Label> {
        &mut self.sinks
    }

    #[must_use]
    pub fn color(&self) -> Color {
        self.color
    }

    pub fn recolor(&mut self, color: Color) {
        self.color = color;
    }

    #[must_use]
    pub fn width(&self) -> f32 {
        self.width
    }

    pub fn rewidth(&mut self, width: f32) {
        self.width = width.abs();
    }
}
