use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize, de::DeserializeOwned};

use state::particle::Particle;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Label(pub usize);

impl From<Label> for String {
    fn from(label: Label) -> Self {
        format!("{}", label.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(bound(serialize = "T: Serialize", deserialize = "T: DeserializeOwned"))]
pub struct Node<T: Clone + Eq + Ord + Serialize + DeserializeOwned> {
    pub label: Label,
    pub particle: Particle<T>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(bound(serialize = "T: Serialize", deserialize = "T: DeserializeOwned"))]
pub struct Edge<T: Clone + Eq + Ord + Serialize + DeserializeOwned> {
    pub label: Label,
    pub inference: relation::Edge<BTreeSet<Label>>,
    pub relation: relation::Edge<T>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct Meta {}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(bound(serialize = "T: Serialize", deserialize = "T: DeserializeOwned"))]
pub struct Hypergraph<T: Clone + Eq + Ord + Serialize + DeserializeOwned> {
    #[serde(rename = "_meta")]
    pub meta: Meta,
    pub nodes: BTreeSet<Node<T>>,
    pub edges: BTreeSet<Edge<state::wave::Wave<T>>>,
    pub particles: usize,
    pub refractions: BTreeMap<Label, Label>,
    pub world: BTreeMap<Label, usize>,
    pub worlds: usize,
    pub united: BTreeMap<Label, BTreeSet<Label>>,
    pub future: BTreeMap<Label, BTreeSet<Label>>,
    pub past: BTreeMap<Label, BTreeSet<Label>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Inference {
    pub edges: BTreeSet<Label>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Translation {
    Existing(Label),
    New(Label),
}

impl Translation {
    #[must_use]
    pub fn label(&self) -> Label {
        match self {
            Translation::Existing(label) | Translation::New(label) => *label,
        }
    }

    #[must_use]
    pub fn created(&self) -> Option<Label> {
        match self {
            Translation::New(label) => Some(*label),
            Translation::Existing(_) => None,
        }
    }
}
