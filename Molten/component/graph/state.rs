use itertools::Itertools;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
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
                .filter_map(|(particle, count)| {
                    if count > 0 {
                        Some((particle, count))
                    } else {
                        None
                    }
                })
                .collect::<BTreeMap<_, _>>();

            unique.insert(Wave(divergence));
        }

        unique.into_iter().collect()
    }
}
