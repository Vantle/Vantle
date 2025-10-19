pub use lava;
pub use obsidian;

use component::graph::matrix::Related;
use component::graph::state::{Particle, Wave};
use itertools::Itertools;
use miette::Diagnostic;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Label(pub usize);

#[derive(Error, Debug, Diagnostic)]
pub enum Error {
    #[error("Missing {kind}: {label:?}")]
    #[diagnostic(code(inferencing::missing))]
    Missing {
        label: Label,
        kind: String,
        #[help]
        suggestion: String,
    },
}

impl Error {
    fn node(label: Label) -> Self {
        Error::Missing {
            label,
            kind: "Node".to_string(),
            suggestion: "Ensure the node exists in the graph before querying".to_string(),
        }
    }

    fn edge(label: Label) -> Self {
        Error::Missing {
            label,
            kind: "Edge".to_string(),
            suggestion: "Ensure the edge exists in the graph before querying".to_string(),
        }
    }

    fn world(label: Label) -> Self {
        Error::Missing {
            label,
            kind: "World".to_string(),
            suggestion: "Ensure the node has been initialized".to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Node<T: Clone + Eq + Ord> {
    pub label: Label,
    pub particle: Particle<T>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Edge<T: Clone + Eq + Ord> {
    pub label: Label,
    pub sources: BTreeSet<Label>,
    pub refractions: BTreeSet<Label>,
    pub source: Wave<T>,
    pub refraction: Wave<T>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct Meta {}

pub struct Inference {
    pub edges: BTreeSet<Label>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Hypergraph<T: Clone + Eq + Ord> {
    _meta: Meta,
    nodes: BTreeSet<Node<T>>,
    edges: BTreeSet<Edge<T>>,
    particles: usize,
    refractions: BTreeMap<Label, Label>,
    world: BTreeMap<Label, usize>,
    worlds: usize,
    united: BTreeMap<Label, BTreeSet<Label>>,
    future: BTreeMap<Label, BTreeSet<Label>>,
    past: BTreeMap<Label, BTreeSet<Label>>,
}

pub struct Strategy {
    pub breadth: usize,
    pub depth: usize,
}

impl<
        T: Clone + Eq + Ord + std::hash::Hash + serde::Serialize + for<'de> serde::Deserialize<'de>,
    > Hypergraph<T>
{
    pub fn node(&self, label: Label) -> Result<&Node<T>, Error> {
        self.nodes
            .iter()
            .find(|node| node.label == label)
            .ok_or_else(|| Error::node(label))
    }

    pub fn edge(&self, label: Label) -> Result<&Edge<T>, Error> {
        self.edges
            .iter()
            .find(|edge| edge.label == label)
            .ok_or_else(|| Error::edge(label))
    }

    pub fn focus(&mut self, particle: Particle<T>) -> Label {
        let label = Label(self.particles);
        self.particles += 1;

        let node = Node { label, particle };
        self.nodes.insert(node);

        self.refractions.insert(label, label);
        self.world.insert(label, self.worlds);
        self.worlds += 1;
        self.united.insert(label, BTreeSet::from([label]));
        self.future.insert(label, BTreeSet::new());
        self.past.insert(label, BTreeSet::new());

        label
    }

    pub fn diffuse(&mut self, signal: Wave<T>) -> impl Iterator<Item = Label> {
        let labels: Vec<Label> = signal
            .particles()
            .map(|particle| self.focus(particle.clone()))
            .collect();
        labels.into_iter()
    }

    #[allow(unused)]
    fn find(&mut self, label: Label) -> Label {
        if self.refractions.get(&label) != Some(&label) {
            let past = *self.refractions.get(&label).unwrap();
            let present = self.find(past);
            self.refractions.insert(label, present);
            present
        } else {
            label
        }
    }

    #[allow(unused)]
    fn unite(&mut self, first: Label, second: Label) -> Result<Label, Error> {
        let anchor = self.find(first);
        let pivot = self.find(second);

        if anchor != pivot {
            let foundation = *self
                .world
                .get(&anchor)
                .ok_or_else(|| Error::world(anchor))?;
            let elevation = *self.world.get(&pivot).ok_or_else(|| Error::world(pivot))?;

            let (merged, subset) = if foundation < elevation {
                self.refractions.insert(anchor, pivot);
                (pivot, anchor)
            } else if foundation > elevation {
                self.refractions.insert(pivot, anchor);
                (anchor, pivot)
            } else {
                self.refractions.insert(pivot, anchor);
                self.world.insert(anchor, foundation + 1);
                (anchor, pivot)
            };

            if let Some(subset) = self.united.remove(&subset) {
                self.united.entry(merged).or_default().extend(subset);
            }

            Ok(merged)
        } else {
            Ok(anchor)
        }
    }

    pub fn independent(&self, rank: usize) -> impl Iterator<Item = Vec<Label>> + '_ {
        let classes: Vec<Vec<Label>> = self
            .united
            .values()
            .map(|set| set.iter().copied().collect())
            .collect();

        if rank > classes.len() {
            return itertools::Either::Left(std::iter::empty());
        }

        let combinations = classes.into_iter().combinations(rank);

        let result = combinations.flat_map(move |selected_classes| {
            let class_vecs: Vec<Vec<Label>> = selected_classes;
            class_vecs.into_iter().multi_cartesian_product()
        });

        itertools::Either::Right(result)
    }

    pub fn infer(
        &mut self,
        refractions: Related<Wave<T>>,
        _strategy: Strategy,
    ) -> Result<Inference, Error> {
        for (_wave, _refractions) in refractions {}
        Ok(Inference {
            edges: BTreeSet::new(),
        })
    }

    pub fn nodes<F>(&self, filter: F) -> impl Iterator<Item = Label> + '_
    where
        F: Fn(&Node<T>) -> bool + 'static,
    {
        self.nodes
            .iter()
            .filter(move |node| filter(node))
            .map(|node| node.label)
    }

    pub fn edges<F>(&self, filter: F) -> impl Iterator<Item = Label> + '_
    where
        F: Fn(&Edge<T>) -> bool + 'static,
    {
        self.edges
            .iter()
            .filter(move |edge| filter(edge))
            .map(|edge| edge.label)
    }

    pub fn united(&self) -> impl Iterator<Item = impl Iterator<Item = Label> + '_> + '_ {
        self.united.values().map(|set| set.iter().copied())
    }
}
