use itertools::Itertools;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::hash::Hash;
use std::hash::Hasher;

use serde_with::serde_as;

use traits::node::{Queryable, Scaled, Translatable};

#[serde_as]
#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(bound(serialize = "T: Serialize", deserialize = "T: Deserialize<'de>"))]
pub struct Particle<T: Eq + Ord>(#[serde_as(as = "Vec<(_, _)>")] BTreeMap<T, usize>);

#[serde_as]
#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(bound(serialize = "T: Serialize", deserialize = "T: Deserialize<'de>"))]
pub struct Wave<T: Eq + Ord>(#[serde_as(as = "Vec<(_, _)>")] BTreeMap<Particle<T>, usize>);

impl<T: Eq + Ord + Hash> Hash for Particle<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.iter().fold(0usize, |hash, (label, count)| {
            label.hash(state);
            count.hash(state);
            hash
        });
    }
}

impl<T, U> From<&[U]> for Particle<T>
where
    T: Eq + Ord + Hash + From<U>,
    U: Clone,
{
    fn from(elements: &[U]) -> Self {
        Particle(
            elements
                .iter()
                .cloned()
                .map(T::from)
                .counts()
                .into_iter()
                .collect::<BTreeMap<_, _>>(),
        )
    }
}

impl<T: Clone + Eq + Ord> Particle<T> {
    pub fn fundamental(data: T) -> Self {
        Particle(BTreeMap::from_iter([(data, 1)]))
    }
}

impl<T: Clone + Eq + Ord> Scaled for Particle<T> {
    type Value = T;
    type Magnitude = usize;

    fn scale(&self, basis: &Self::Value) -> Self::Magnitude {
        self.0.get(basis).copied().unwrap_or(0)
    }
}

impl<T: Clone + Eq + Ord> Queryable for Particle<T> {
    fn subset(&self, basis: &Self) -> Option<&Self> {
        self.0
            .iter()
            .all(|(key, &count)| basis.0.get(key).is_some_and(|&relative| relative <= count))
            .then_some(self)
    }

    fn superset(&self, basis: &Self) -> Option<&Self> {
        basis
            .0
            .iter()
            .all(|(key, &count)| self.0.get(key).is_some_and(|&relative| relative >= count))
            .then_some(self)
    }

    fn joint(&self, basis: &Self) -> Option<&Self> {
        self.0
            .keys()
            .any(|key| basis.0.contains_key(key))
            .then_some(self)
    }

    fn disjoint(&self, basis: &Self) -> Option<&Self> {
        self.0
            .keys()
            .all(|key| !basis.0.contains_key(key))
            .then_some(self)
    }

    fn isomorphic(&self, basis: &Self) -> Option<&Self> {
        (self.0 == basis.0).then_some(self)
    }
}

impl<T: Clone + Eq + Ord> Translatable for Particle<T> {
    fn join(&self, basis: &Self) -> Option<Self> {
        let mut modified = false;
        let mut result = self.0.clone();

        for (key, value) in &basis.0 {
            if *value > 0 {
                modified = true;
            }
            *result.entry(key.clone()).or_insert(0) += value;
        }

        if !modified {
            return None;
        }

        Some(Particle(result))
    }

    fn intersect(&self, basis: &Self) -> Option<Self> {
        let intersection: BTreeMap<T, usize> = self
            .0
            .iter()
            .filter_map(|(key, &count)| {
                basis
                    .0
                    .get(key)
                    .map(|&relative| (key.clone(), count.min(relative)))
            })
            .filter(|(_, count)| *count > 0)
            .collect();

        (!intersection.is_empty()).then_some(Particle(intersection))
    }

    fn diverge(&self, basis: &Self) -> Option<Self> {
        let difference: BTreeMap<T, usize> = self
            .0
            .iter()
            .filter_map(|(key, &count)| match basis.0.get(key) {
                Some(&relative) if count > relative => Some((key.clone(), count - relative)),
                None => Some((key.clone(), count)),
                _ => None,
            })
            .collect();

        (!difference.is_empty()).then_some(Particle(difference))
    }
}

impl<T: Eq + Ord + Hash> Hash for Wave<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.iter().fold(0usize, |hash, (packet, count)| {
            packet.hash(state);
            count.hash(state);
            hash
        });
    }
}

impl<T: Eq + Ord + Hash + Clone> From<&[Particle<T>]> for Wave<T> {
    fn from(elements: &[Particle<T>]) -> Self {
        Wave(
            elements
                .iter()
                .cloned()
                .counts()
                .into_iter()
                .collect::<BTreeMap<_, _>>(),
        )
    }
}

impl<T: Clone + Eq + Ord> Wave<T> {
    pub fn monochromatic(data: Particle<T>) -> Self {
        Wave(BTreeMap::from_iter([(data, 1)]))
    }
}

impl<T: Clone + Eq + Ord> Scaled for Wave<T> {
    type Value = Particle<T>;
    type Magnitude = usize;

    fn scale(&self, basis: &Self::Value) -> Self::Magnitude {
        self.0.get(basis).copied().unwrap_or(0)
    }
}

impl<T: Clone + Eq + Ord + Hash> Queryable for Wave<T> {
    fn subset(&self, basis: &Self) -> Option<&Self> {
        // Early return for empty self
        if self.0.is_empty() {
            return Some(self);
        }

        // Collect all unique particles and assign indices
        let particles: Vec<&Particle<T>> = self.0.keys().chain(basis.0.keys()).unique().collect();
        let nodes = particles.len() + 2; // +2 for source and sink
        let source = 0;
        let sink = 1;
        let offset = 2; // particle indices start at 2

        // Build adjacency list for flow network
        let mut edges = flow::new(nodes);

        // Add edges from source to self particles
        for (i, particle) in particles.iter().enumerate() {
            if let Some(&count) = self.0.get(particle) {
                flow::add_edge(&mut edges, source, i + offset, count);
            }
        }

        // Add edges between compatible particles
        for (i, from) in particles.iter().enumerate() {
            if self.0.contains_key(from) {
                for (j, to) in particles.iter().enumerate() {
                    if basis.0.contains_key(to) && from.subset(to).is_some() {
                        flow::add_edge(&mut edges, i + offset, j + offset, usize::MAX);
                    }
                }
            }
        }

        // Add edges from basis particles to sink
        for (i, particle) in particles.iter().enumerate() {
            if let Some(&count) = basis.0.get(particle) {
                flow::add_edge(&mut edges, i + offset, sink, count);
            }
        }

        // Calculate total demand
        let demand: usize = self.0.values().sum();

        // Run max flow
        let flow = flow::maxflow(&mut edges, source, sink);

        (flow == demand).then_some(self)
    }

    fn superset(&self, _basis: &Self) -> Option<&Self> {
        todo!()
    }

    fn joint(&self, _basis: &Self) -> Option<&Self> {
        todo!()
    }

    fn disjoint(&self, _basis: &Self) -> Option<&Self> {
        todo!()
    }

    fn isomorphic(&self, _basis: &Self) -> Option<&Self> {
        todo!()
    }
}

// Flow network for bipartite matching
mod flow {
    pub struct Edge {
        pub to: usize,
        pub capacity: usize,
        pub flow: usize,
        pub reverse: usize,
    }

    pub fn add_edge(edges: &mut [Vec<Edge>], from: usize, to: usize, capacity: usize) {
        let forward = edges[from].len();
        let reverse = edges[to].len();

        edges[from].push(Edge {
            to,
            capacity,
            flow: 0,
            reverse,
        });

        edges[to].push(Edge {
            to: from,
            capacity: 0,
            flow: 0,
            reverse: forward,
        });
    }

    pub fn maxflow(edges: &mut [Vec<Edge>], source: usize, sink: usize) -> usize {
        let mut total = 0;
        let nodes = edges.len();

        loop {
            // Find augmenting path using DFS
            let mut parent = vec![None; nodes];
            let mut visited = vec![false; nodes];

            if !dfs(edges, source, sink, &mut visited, &mut parent) {
                break;
            }

            // Find minimum capacity along the path
            let mut flow = usize::MAX;
            let mut current = sink;

            while let Some((prev, edge)) = parent[current] {
                flow = flow.min(edges[prev][edge].capacity - edges[prev][edge].flow);
                current = prev;
            }

            // Update flow along the path
            current = sink;
            while let Some((prev, edge)) = parent[current] {
                edges[prev][edge].flow += flow;
                let reverse = edges[prev][edge].reverse;
                edges[current][reverse].flow = edges[current][reverse].flow.saturating_sub(flow);
                current = prev;
            }

            total += flow;
        }

        total
    }

    fn dfs(
        edges: &[Vec<Edge>],
        current: usize,
        sink: usize,
        visited: &mut [bool],
        parent: &mut [Option<(usize, usize)>],
    ) -> bool {
        if current == sink {
            return true;
        }

        visited[current] = true;

        for (index, edge) in edges[current].iter().enumerate() {
            if !visited[edge.to] && edge.flow < edge.capacity {
                parent[edge.to] = Some((current, index));
                if dfs(edges, edge.to, sink, visited, parent) {
                    return true;
                }
            }
        }

        false
    }

    pub type Graph = Vec<Vec<Edge>>;

    pub fn new(nodes: usize) -> Graph {
        (0..nodes).map(|_| Vec::new()).collect()
    }
}

impl<T: Clone + Eq + Ord> Translatable for Wave<T> {
    fn join(&self, _basis: &Self) -> Option<Self> {
        todo!()
    }

    fn intersect(&self, _basis: &Self) -> Option<Self> {
        todo!()
    }

    fn diverge(&self, _basis: &Self) -> Option<Self> {
        todo!()
    }
}
