use itertools::Itertools;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::hash::Hash;
use std::hash::Hasher;

use serde_with::serde_as;

use traits::node::{Polytranslatable, Queryable, Scaled, Translatable};

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
        for (label, count) in &self.0 {
            label.hash(state);
            count.hash(state);
        }
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

    pub fn empty() -> Self {
        Particle(BTreeMap::new())
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn elements(&self) -> impl Iterator<Item = (&T, &usize)> {
        self.0.iter()
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
        for (packet, count) in &self.0 {
            packet.hash(state);
            count.hash(state);
        }
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

    pub fn polychromatic(data: Particle<T>, multiplicity: usize) -> Self {
        Wave(BTreeMap::from_iter([(data, multiplicity)]))
    }

    pub fn dark() -> Self {
        Wave(BTreeMap::new())
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn particles(&self) -> impl Iterator<Item = &Particle<T>> {
        self.0.keys()
    }

    pub fn count(&self) -> usize {
        self.0.values().sum()
    }

    fn bipartite(
        sources: &[(Particle<T>, usize)],
        sinks: &[(Particle<T>, usize)],
        compatible: impl Fn(&Particle<T>, &Particle<T>) -> bool,
    ) -> bool {
        let mut assignment = vec![None; sources.len()];
        let mut used = vec![false; sinks.len()];

        fn search<T: Clone + Eq + Ord>(
            index: usize,
            sources: &[(Particle<T>, usize)],
            sinks: &[(Particle<T>, usize)],
            compatible: &impl Fn(&Particle<T>, &Particle<T>) -> bool,
            assignment: &mut [Option<usize>],
            used: &mut [bool],
        ) -> bool {
            if index == sources.len() {
                return sources.iter().enumerate().all(|(i, (_, required))| {
                    assignment[i]
                        .map(|j| sinks[j].1 >= *required)
                        .unwrap_or(false)
                });
            }

            let (particle, required) = &sources[index];

            for (slot, (target, available)) in sinks.iter().enumerate() {
                if !used[slot] && available >= required && compatible(particle, target) {
                    used[slot] = true;
                    assignment[index] = Some(slot);

                    if search(index + 1, sources, sinks, compatible, assignment, used) {
                        return true;
                    }

                    used[slot] = false;
                    assignment[index] = None;
                }
            }

            false
        }

        search(0, sources, sinks, &compatible, &mut assignment, &mut used)
    }

    fn enumerate(
        sources: &[(Particle<T>, usize)],
        sinks: &[(Particle<T>, usize)],
        compatible: impl Fn(&Particle<T>, &Particle<T>) -> bool,
    ) -> Vec<Vec<usize>> {
        let mut assignment = vec![None; sources.len()];
        let mut used = vec![false; sinks.len()];
        let mut results: Vec<Vec<usize>> = Vec::new();

        fn explore<T: Clone + Eq + Ord>(
            index: usize,
            sources: &[(Particle<T>, usize)],
            sinks: &[(Particle<T>, usize)],
            test: &impl Fn(&Particle<T>, &Particle<T>) -> bool,
            assignment: &mut [Option<usize>],
            used: &mut [bool],
            results: &mut Vec<Vec<usize>>,
        ) {
            if index == sources.len() {
                if sources.iter().enumerate().all(|(i, (_, needed))| {
                    assignment[i]
                        .map(|j| sinks[j].1 >= *needed)
                        .unwrap_or(false)
                }) {
                    results.push(assignment.iter().map(|m| m.unwrap()).collect());
                }
                return;
            }

            let (particle, needed) = &sources[index];

            for (slot, (target, available)) in sinks.iter().enumerate() {
                if !used[slot] && available >= needed && test(particle, target) {
                    used[slot] = true;
                    assignment[index] = Some(slot);
                    explore(index + 1, sources, sinks, test, assignment, used, results);
                    used[slot] = false;
                    assignment[index] = None;
                }
            }
        }

        explore(
            0,
            sources,
            sinks,
            &compatible,
            &mut assignment,
            &mut used,
            &mut results,
        );
        results
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
        if self.0.is_empty() {
            return Some(self);
        }

        let sources: Vec<_> = self
            .0
            .iter()
            .map(|(particle, count)| (particle.clone(), *count))
            .collect();
        let sinks: Vec<_> = basis
            .0
            .iter()
            .map(|(particle, count)| (particle.clone(), *count))
            .collect();

        Self::bipartite(&sources, &sinks, |source, sink| {
            source.subset(sink).is_some()
        })
        .then_some(self)
    }

    fn superset(&self, basis: &Self) -> Option<&Self> {
        if basis.0.is_empty() {
            return Some(self);
        }

        let sources: Vec<_> = basis
            .0
            .iter()
            .map(|(particle, count)| (particle.clone(), *count))
            .collect();
        let sinks: Vec<_> = self
            .0
            .iter()
            .map(|(particle, count)| (particle.clone(), *count))
            .collect();

        Self::bipartite(&sources, &sinks, |source, sink| {
            source.subset(sink).is_some()
        })
        .then_some(self)
    }

    fn joint(&self, basis: &Self) -> Option<&Self> {
        self.0
            .keys()
            .any(|left| basis.0.keys().any(|right| left.joint(right).is_some()))
            .then_some(self)
    }

    fn disjoint(&self, basis: &Self) -> Option<&Self> {
        let disjoint = self
            .0
            .keys()
            .all(|left| basis.0.keys().all(|right| left.disjoint(right).is_some()));

        disjoint.then_some(self)
    }

    fn isomorphic(&self, basis: &Self) -> Option<&Self> {
        (self.0 == basis.0).then_some(self)
    }
}

impl<T: Clone + Eq + Ord> Translatable for Wave<T> {
    fn join(&self, basis: &Self) -> Option<Self> {
        let mut modified = false;
        let mut result = self.0.clone();

        for (particle, &count) in &basis.0 {
            if count > 0 {
                modified = true;
            }
            *result.entry(particle.clone()).or_insert(0) += count;
        }

        if !modified {
            return None;
        }

        Some(Wave(result))
    }

    fn intersect(&self, basis: &Self) -> Option<Self> {
        let mut result: BTreeMap<Particle<T>, usize> = BTreeMap::new();

        for (left, &lefts) in &self.0 {
            for (right, &rights) in &basis.0 {
                if let Some(common) = left.intersect(right) {
                    let count = lefts.min(rights);
                    if count > 0 {
                        *result.entry(common).or_insert(0) += count;
                    }
                }
            }
        }

        (!result.is_empty()).then_some(Wave(result))
    }

    fn diverge(&self, basis: &Self) -> Option<Self> {
        let difference: BTreeMap<Particle<T>, usize> = self
            .0
            .iter()
            .filter_map(|(particle, &count)| match basis.0.get(particle) {
                Some(&relative) if count > relative => Some((particle.clone(), count - relative)),
                None => Some((particle.clone(), count)),
                _ => None,
            })
            .collect();

        (!difference.is_empty()).then_some(Wave(difference))
    }
}

impl<T: Clone + Eq + Ord> Polytranslatable for Wave<T> {
    type Sequence = Vec<Self>;

    fn diverges(&self, basis: &Self) -> Self::Sequence {
        if basis.0.is_empty() {
            return vec![self.clone()];
        }

        let sources: Vec<_> = basis
            .0
            .iter()
            .map(|(particle, count)| (particle.clone(), *count))
            .collect();
        let sinks: Vec<_> = self
            .0
            .iter()
            .map(|(particle, count)| (particle.clone(), *count))
            .collect();

        let assignments = Self::enumerate(&sources, &sinks, |source, sink| {
            source.subset(sink).is_some()
        });

        let mut unique = std::collections::BTreeSet::new();

        for assignment in assignments {
            let mut remaining: BTreeMap<Particle<T>, usize> =
                sinks.iter().map(|(p, c)| (p.clone(), *c)).collect();

            for (i, &j) in assignment.iter().enumerate() {
                let required = sources[i].1;
                let existing = remaining.get_mut(&sinks[j].0).unwrap();
                *existing -= required;
            }

            let divergence = remaining
                .into_iter()
                .filter(|(_, count)| *count > 0)
                .collect::<BTreeMap<_, _>>();

            unique.insert(Wave(divergence));
        }

        unique.into_iter().collect()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Label(usize);

impl std::fmt::Display for Label {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#{}", self.0)
    }
}

#[serde_as]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(bound(serialize = "T: Serialize", deserialize = "T: Deserialize<'de>"))]
pub struct Inference<T: Clone + Eq + Ord> {
    #[serde_as(as = "Vec<(_, _)>")]
    nodes: BTreeMap<Label, Particle<T>>,
    #[serde_as(as = "Vec<(_, _)>")]
    adjacency: BTreeMap<Label, BTreeSet<Label>>,
    #[serde_as(as = "Vec<(_, _)>")]
    reachability: BTreeMap<Label, BTreeSet<Label>>,
    counter: usize,
}

impl<T: Clone + Eq + Ord> Default for Inference<T> {
    fn default() -> Self {
        Self {
            nodes: BTreeMap::new(),
            adjacency: BTreeMap::new(),
            reachability: BTreeMap::new(),
            counter: 0,
        }
    }
}

impl<T: Clone + Eq + Ord + Hash> Inference<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    pub fn particles(&self) -> impl Iterator<Item = &Particle<T>> {
        self.nodes.values()
    }

    pub fn labels(&self) -> impl Iterator<Item = Label> + '_ {
        self.nodes.keys().copied()
    }

    pub fn contains(&self, particle: &Particle<T>) -> bool {
        self.nodes.values().any(|p| p == particle)
    }

    pub fn node(&self, id: &Label) -> Option<&Particle<T>> {
        self.nodes.get(id)
    }

    pub fn nodes(&self, particle: &Particle<T>) -> Vec<Label> {
        self.nodes
            .iter()
            .filter(|(_, p)| *p == particle)
            .map(|(id, _)| *id)
            .collect()
    }

    pub fn entries(&self) -> impl Iterator<Item = (Label, &Particle<T>)> + '_ {
        self.nodes.iter().map(|(id, p)| (*id, p))
    }

    pub fn edges(&self, source: &Label) -> Option<impl Iterator<Item = Label> + '_> {
        self.adjacency.get(source).map(|set| set.iter().copied())
    }

    pub fn insert(&mut self, particle: Particle<T>) -> Label {
        let id = Label(self.counter);
        self.counter += 1;
        self.nodes.insert(id, particle);
        self.adjacency.insert(id, BTreeSet::new());
        self.reachability.insert(id, BTreeSet::new());
        id
    }

    pub fn reaches(&self, source: &Label, target: &Label) -> bool {
        self.reachability
            .get(source)
            .map(|r| r.contains(target))
            .unwrap_or(false)
    }

    fn predecessors(&self, node: &Label) -> Vec<Label> {
        self.adjacency
            .keys()
            .filter(|&id| self.reaches(id, node))
            .copied()
            .collect()
    }

    pub fn connect(&mut self, source: Label, target: Label) -> bool {
        if !self.nodes.contains_key(&source) || !self.nodes.contains_key(&target) {
            return false;
        }

        if self.connected(&source, &target) {
            return false;
        }

        self.adjacency.get_mut(&source).unwrap().insert(target);

        let mut reachable_from_target = self.reachability[&target].clone();
        reachable_from_target.insert(target);

        let mut sources_to_update = self.predecessors(&source);
        sources_to_update.push(source);

        for from in sources_to_update {
            let reach = self.reachability.get_mut(&from).unwrap();
            reach.extend(reachable_from_target.iter().copied());
        }

        true
    }

    pub fn remove(&mut self, node: &Label) {
        if !self.nodes.contains_key(node) {
            return;
        }

        self.nodes.remove(node);
        self.adjacency.remove(node);
        self.reachability.remove(node);

        for neighbors in self.adjacency.values_mut() {
            neighbors.remove(node);
        }

        for reachable in self.reachability.values_mut() {
            reachable.remove(node);
        }

        self.rebuild();
    }

    fn rebuild(&mut self) {
        for reachable in self.reachability.values_mut() {
            reachable.clear();
        }

        let node_ids: Vec<_> = self.adjacency.keys().copied().collect();

        for source in &node_ids {
            let mut visited = BTreeSet::new();
            let mut stack = vec![*source];

            while let Some(current) = stack.pop() {
                if visited.contains(&current) {
                    continue;
                }
                visited.insert(current);

                if current != *source {
                    self.reachability.get_mut(source).unwrap().insert(current);
                }

                if let Some(neighbors) = self.adjacency.get(&current) {
                    for &neighbor in neighbors {
                        if !visited.contains(&neighbor) {
                            stack.push(neighbor);
                        } else if neighbor == *source && current != *source {
                            self.reachability.get_mut(source).unwrap().insert(*source);
                        }
                    }
                }
            }
        }
    }

    pub fn independent(&self, left: &Label, right: &Label) -> bool {
        left != right && !self.reaches(left, right) && !self.reaches(right, left)
    }

    pub fn independents(&self, node: &Label) -> Vec<Label> {
        self.adjacency
            .keys()
            .filter(|&other| self.independent(node, other))
            .copied()
            .collect()
    }

    pub fn independence(&self, nodes: &[Label]) -> bool {
        nodes
            .iter()
            .enumerate()
            .all(|(i, n1)| nodes[..i].iter().all(|n2| self.independent(n1, n2)))
    }

    pub fn neighbors(&self, node: &Label) -> Option<&BTreeSet<Label>> {
        self.adjacency.get(node)
    }

    pub fn successors(&self, node: &Label) -> Option<&BTreeSet<Label>> {
        self.reachability.get(node)
    }

    pub fn connected(&self, source: &Label, target: &Label) -> bool {
        self.adjacency
            .get(source)
            .map(|neighbors| neighbors.contains(target))
            .unwrap_or(false)
    }

    pub fn cyclic(&self, node: &Label) -> bool {
        self.reaches(node, node)
    }

    pub fn cycles(&self) -> Vec<Label> {
        self.adjacency
            .keys()
            .filter(|&id| self.cyclic(id))
            .copied()
            .collect()
    }

    pub fn matching(&self, wave: &Wave<T>) -> Vec<Vec<Label>> {
        if wave.is_empty() {
            return self
                .nodes
                .iter()
                .filter(|(_, p)| p.is_empty())
                .map(|(id, _)| vec![*id])
                .collect();
        }

        let measured: Vec<_> = wave
            .0
            .iter()
            .flat_map(|(particle, &count)| std::iter::repeat_n(particle.clone(), count))
            .collect();

        let mut results = Vec::new();

        fn search<T: Clone + Eq + Ord + Hash>(
            inference: &Inference<T>,
            measured: &[Particle<T>],
            index: usize,
            current: &mut Vec<Label>,
            used: &mut BTreeSet<Label>,
            results: &mut Vec<Vec<Label>>,
        ) {
            if index == measured.len() {
                results.push(current.clone());
                return;
            }

            let target = &measured[index];

            for (&id, particle) in &inference.nodes {
                if used.contains(&id) || target.subset(particle).is_none() {
                    continue;
                }

                if current
                    .iter()
                    .all(|previous| inference.independent(previous, &id))
                {
                    current.push(id);
                    used.insert(id);

                    search(inference, measured, index + 1, current, used, results);

                    current.pop();
                    used.remove(&id);
                }
            }
        }

        search(
            self,
            &measured,
            0,
            &mut Vec::new(),
            &mut BTreeSet::new(),
            &mut results,
        );
        results
    }

    pub fn infer(&mut self, matched: &[Label], destinations: &[Particle<T>]) -> bool {
        let mut changed = false;
        let mut evolutions = Vec::new();

        let preexisting: BTreeMap<&Particle<T>, Vec<Label>> = destinations
            .iter()
            .map(|dest| (dest, self.nodes(dest)))
            .collect();

        for destination in destinations {
            let existing = &preexisting[destination];

            if existing.is_empty() {
                let id = self.insert(destination.clone());
                evolutions.push(id);
                changed = true;
            } else {
                evolutions.extend(existing.iter().copied());
            }
        }

        for &source in matched {
            for &destination in &evolutions {
                if self.connect(source, destination) {
                    changed = true;
                }
            }
        }

        changed
    }

    pub fn join(&mut self, left: &Label, right: &Label) -> Option<Label> {
        if !self.nodes.contains_key(left) || !self.nodes.contains_key(right) {
            return None;
        }

        if left == right {
            return Some(*left);
        }

        let past = self.nodes[left].clone();
        let future = self.nodes[right].clone();

        let joined = past.join(&future)?;

        let creation = self.insert(joined);

        let pasts = self.predecessors(left);
        let futures = self.predecessors(right);

        for predecessor in pasts {
            self.connect(predecessor, creation);
        }

        for predecessor in futures {
            self.connect(predecessor, creation);
        }

        Some(creation)
    }
}
