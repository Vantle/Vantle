use std::collections::{BTreeMap, BTreeSet};
use std::hash::Hash;

use observe::trace;
use serde::{de::DeserializeOwned, Serialize};

use component::graph::relation::Related;
use component::graph::state::particle::Particle;
use component::graph::state::wave::Wave;

pub trait Set {
    fn subset(&self, basis: &Self) -> Option<&Self>;
    fn superset(&self, basis: &Self) -> Option<&Self>;
    fn joint(&self, basis: &Self) -> Option<&Self>;
    fn disjoint(&self, basis: &Self) -> Option<&Self>;
    fn isomorphic(&self, basis: &Self) -> Option<&Self>;
}

pub trait Ranked {
    fn rank(&self) -> usize;

    fn empty(&self) -> bool {
        self.rank() == 0
    }
}

pub trait Polyset
where
    Self: Sized,
{
    type Sequence: IntoIterator<Item = Self>;
    fn diverges(&self, basis: &Self) -> Self::Sequence;
}

pub trait Searchable<Element> {
    type Target;

    fn contains(&self, element: &Element) -> bool;
    fn find(&self, element: &Element) -> Option<Self::Target>;
}

impl<Label: Eq + Ord + Serialize + DeserializeOwned> Ranked for Related<Label> {
    #[trace(channels = [core])]
    fn rank(&self) -> usize {
        self.adjacency.len()
    }
}

impl<Label: Eq + Ord + Clone + Serialize + DeserializeOwned> Searchable<Label> for Related<Label> {
    type Target = BTreeSet<Label>;

    #[trace(channels = [core])]
    fn contains(&self, element: &Label) -> bool {
        self.adjacency.contains_key(element)
    }

    #[trace(channels = [core])]
    fn find(&self, element: &Label) -> Option<Self::Target> {
        self.adjacency.get(element).cloned()
    }
}

impl<T: Eq + Ord> Ranked for Particle<T> {
    #[trace(channels = [core])]
    fn rank(&self) -> usize {
        self.elements.len()
    }
}

impl<T: Clone + Eq + Ord> Set for Particle<T> {
    #[trace(channels = [core])]
    fn subset(&self, basis: &Self) -> Option<&Self> {
        self.elements
            .iter()
            .all(|(key, &count)| {
                basis
                    .elements
                    .get(key)
                    .is_some_and(|&relative| count <= relative)
            })
            .then_some(self)
    }

    #[trace(channels = [core])]
    fn superset(&self, basis: &Self) -> Option<&Self> {
        basis
            .elements
            .iter()
            .all(|(key, &count)| {
                self.elements
                    .get(key)
                    .is_some_and(|&relative| relative >= count)
            })
            .then_some(self)
    }

    #[trace(channels = [core])]
    fn joint(&self, basis: &Self) -> Option<&Self> {
        self.elements
            .keys()
            .any(|key| basis.elements.contains_key(key))
            .then_some(self)
    }

    #[trace(channels = [core])]
    fn disjoint(&self, basis: &Self) -> Option<&Self> {
        self.elements
            .keys()
            .all(|key| !basis.elements.contains_key(key))
            .then_some(self)
    }

    #[trace(channels = [core])]
    fn isomorphic(&self, basis: &Self) -> Option<&Self> {
        (self.elements == basis.elements).then_some(self)
    }
}

impl<T: Eq + Ord> Ranked for Wave<T> {
    #[trace(channels = [core])]
    fn rank(&self) -> usize {
        self.particles.values().sum()
    }
}

#[must_use]
#[trace(channels = [core])]
pub fn bipartite<T, F>(source: &Wave<T>, sink: &Wave<T>, compatible: F) -> bool
where
    T: Clone + Eq + Ord + Hash,
    F: Fn(&Particle<T>, &Particle<T>) -> bool,
{
    let sources = sink
        .into_iter()
        .map(|(p, c)| (p.clone(), *c))
        .collect::<Vec<_>>();
    let sinks = source
        .into_iter()
        .map(|(p, c)| (p.clone(), *c))
        .collect::<Vec<_>>();

    fn search<T, F>(
        index: usize,
        sources: &[(Particle<T>, usize)],
        sinks: &[(Particle<T>, usize)],
        assignments: &mut [usize],
        compatible: &F,
    ) -> bool
    where
        T: Clone + Eq + Ord,
        F: Fn(&Particle<T>, &Particle<T>) -> bool,
    {
        if index == sources.len() {
            return true;
        }

        let (particle, required) = &sources[index];

        for (slot, (target, available)) in sinks.iter().enumerate() {
            if let Some(remaining) = available.checked_sub(assignments[slot]) {
                if remaining >= *required && compatible(particle, target) {
                    assignments[slot] += required;

                    if search(index + 1, sources, sinks, assignments, compatible) {
                        return true;
                    }

                    assignments[slot] -= required;
                }
            }
        }

        false
    }

    search(0, &sources, &sinks, &mut vec![0; sinks.len()], &compatible)
}

fn overlaps<T: Eq + Ord>(left: &Particle<T>, right: &Particle<T>) -> bool {
    left.elements
        .keys()
        .any(|key| right.elements.contains_key(key))
}

impl<T: Clone + Eq + Ord + Hash> Set for Wave<T> {
    #[trace(channels = [core])]
    fn subset(&self, basis: &Self) -> Option<&Self> {
        if self.particles.is_empty() {
            return Some(self);
        }

        if basis.particles.is_empty() {
            return None;
        }

        bipartite(basis, self, |left, right| left.subset(right).is_some()).then_some(self)
    }

    #[trace(channels = [core])]
    fn superset(&self, basis: &Self) -> Option<&Self> {
        if basis.particles.is_empty() {
            return Some(self);
        }

        if self.particles.is_empty() {
            return None;
        }

        bipartite(self, basis, |left, right| left.subset(right).is_some()).then_some(self)
    }

    #[trace(channels = [core])]
    fn joint(&self, basis: &Self) -> Option<&Self> {
        self.particles
            .keys()
            .any(|left| basis.particles.keys().any(|right| overlaps(left, right)))
            .then_some(self)
    }

    #[trace(channels = [core])]
    fn disjoint(&self, basis: &Self) -> Option<&Self> {
        self.particles
            .keys()
            .all(|left| basis.particles.keys().all(|right| !overlaps(left, right)))
            .then_some(self)
    }

    #[trace(channels = [core])]
    fn isomorphic(&self, basis: &Self) -> Option<&Self> {
        (self.particles == basis.particles).then_some(self)
    }
}

impl<T: Clone + Eq + Ord + Hash> Polyset for Wave<T> {
    type Sequence = Vec<Wave<T>>;

    #[trace(channels = [core])]
    fn diverges(&self, basis: &Self) -> Self::Sequence {
        if basis.particles.is_empty() {
            return vec![self.clone()];
        }

        let sources = basis
            .into_iter()
            .map(|(p, c)| (p.clone(), *c))
            .collect::<Vec<_>>();
        let sinks = self
            .into_iter()
            .map(|(p, c)| (p.clone(), *c))
            .collect::<Vec<_>>();

        fn search<T: Clone + Eq + Ord>(
            index: usize,
            sources: &[(Particle<T>, usize)],
            sinks: &[(Particle<T>, usize)],
            assignment: &mut [Option<usize>],
            used: &mut [bool],
            results: &mut BTreeSet<Wave<T>>,
        ) {
            if index == sources.len() {
                if sources
                    .iter()
                    .enumerate()
                    .all(|(i, (_, needed))| assignment[i].is_some_and(|j| sinks[j].1 >= *needed))
                {
                    let mut remaining: BTreeMap<Particle<T>, usize> =
                        sinks.iter().map(|(p, c)| (p.clone(), *c)).collect();

                    for (i, &j) in assignment.iter().enumerate() {
                        let Some(slot) = j else { continue };
                        let required = sources[i].1;
                        if let Some(existing) = remaining.get_mut(&sinks[slot].0) {
                            *existing -= required;
                        }
                    }

                    let divergence = remaining
                        .into_iter()
                        .filter(|(_, count)| *count > 0usize)
                        .collect::<BTreeMap<_, _>>();

                    results.insert(Wave::new(divergence));
                }
                return;
            }

            let (particle, needed) = &sources[index];

            for (slot, (candidate, available)) in sinks.iter().enumerate() {
                if !used[slot] && available >= needed && particle.subset(candidate).is_some() {
                    used[slot] = true;
                    assignment[index] = Some(slot);
                    search(index + 1, sources, sinks, assignment, used, results);
                    used[slot] = false;
                    assignment[index] = None;
                }
            }
        }

        let mut unique = BTreeSet::new();
        search(
            0,
            &sources,
            &sinks,
            &mut vec![None; sources.len()],
            &mut vec![false; sinks.len()],
            &mut unique,
        );

        unique.into_iter().collect()
    }
}
