use std::collections::{BTreeMap, BTreeSet};
use std::hash::Hash;

use itertools::Itertools;
use observe::trace;
use record::category::State;
use serde::{Serialize, de::DeserializeOwned};

use component::graph::relation::Edge as Relation;
use component::graph::relation::Related as Relations;
use component::graph::state::particle::Particle as Particulate;
use component::graph::state::wave::Wave as Waveform;
use component::hypergraph::{Edge, Hypergraph, Inference, Label, Node, Translation};
use error::{Error, Result};
use query::{Polyset, Ranked, Set};
use state::particle::Particle;
use state::wave::{Wave, matchings};

pub use error;

pub fn bipartitions<W>(source: &W, sink: &W) -> impl Iterator<Item = W>
where
    W: Wave + Clone + Eq + Ord + Hash + Serialize,
    W::Particle: Particle + Clone,
{
    if sink.empty() {
        return itertools::Either::Left(std::iter::once(source.clone()));
    }

    let unique = matchings(source, sink).collect::<BTreeSet<W>>();
    itertools::Either::Right(unique.into_iter())
}

fn ancestors<V: Clone + Eq + Ord + Hash + Serialize + DeserializeOwned>(
    label: Label,
    past: &BTreeMap<Label, BTreeSet<Label>>,
    graph: &Hypergraph<V>,
) -> Vec<Label>
where
    Particulate<V>: Set + Ranked,
{
    let mut result = Vec::new();
    let mut visited = BTreeSet::from([label]);
    let mut queue = vec![label];

    while let Some(current) = queue.pop() {
        if let Some(incoming) = past.get(&current) {
            for &edge in incoming {
                if let Ok(found) = graph.edge(edge) {
                    for &source in &found.inference.source {
                        if visited.insert(source) {
                            result.push(source);
                            queue.push(source);
                        }
                    }
                }
            }
        }
    }

    result
}

fn chains<V: Clone + Eq + Ord + Hash + Serialize + DeserializeOwned>(
    sources: &[Label],
    past: &BTreeMap<Label, BTreeSet<Label>>,
    graph: &Hypergraph<V>,
) -> Vec<Vec<Label>>
where
    Particulate<V>: Set + Ranked,
{
    sources
        .iter()
        .map(|&source| {
            let mut chain = vec![source];
            chain.extend(ancestors(source, past, graph));
            chain
        })
        .collect()
}

pub trait Evaluate {
    type Value: Clone + Eq + Ord + Hash + Serialize + DeserializeOwned;
    type Particle;
    type Wave: Wave + Clone + Eq + Ord + Serialize + DeserializeOwned;
    type Error;

    fn node(&self, label: Label) -> std::result::Result<&Node<Self::Value>, Self::Error>;
    fn edge(&self, label: Label) -> std::result::Result<&Edge<Self::Wave>, Self::Error>;

    fn focus(&mut self, particle: Self::Particle) -> Label;
    fn diffuse(&mut self, signal: Self::Wave) -> impl Iterator<Item = Label>;
    fn absorb(
        &mut self,
        source: BTreeSet<Label>,
        relation: Relation<Self::Wave>,
    ) -> std::result::Result<impl Iterator<Item = Label>, Self::Error>;

    fn nodes<F>(&self, filter: F) -> impl Iterator<Item = Label> + '_
    where
        F: Fn(&Node<Self::Value>) -> bool + 'static;
    fn edges<F>(&self, filter: F) -> impl Iterator<Item = Label> + '_
    where
        F: Fn(&Edge<Self::Wave>) -> bool + 'static;
    fn independent(&self, rank: usize) -> impl Iterator<Item = BTreeSet<Label>>;
    fn bipartite(
        &self,
        combination: BTreeSet<Label>,
        rule: &Self::Wave,
    ) -> std::result::Result<impl Iterator<Item = Self::Wave>, Self::Error>;
    fn united(&self) -> impl Iterator<Item = impl Iterator<Item = Label> + '_> + '_;
    fn isomorphics<'a>(&'a self, particle: &'a Self::Particle) -> impl Iterator<Item = Label> + 'a;

    fn locate(&mut self, label: Label) -> std::result::Result<Label, Self::Error>;
    fn unite(&mut self, first: Label, second: Label) -> std::result::Result<Label, Self::Error>;
    fn translate(
        &mut self,
        source: BTreeSet<Label>,
        destinations: BTreeSet<Label>,
        rule: Relation<Self::Wave>,
    ) -> std::result::Result<Translation, Self::Error>;

    fn infer(
        &mut self,
        refractions: Relations<Self::Wave>,
    ) -> std::result::Result<Inference, Self::Error>;

    fn fixed(
        &mut self,
        refractions: Relations<Self::Wave>,
    ) -> std::result::Result<Inference, Self::Error>;
}

impl<T: Clone + Eq + Ord + std::hash::Hash + Serialize + DeserializeOwned> Evaluate
    for Hypergraph<T>
{
    type Value = T;
    type Particle = Particulate<T>;
    type Wave = Waveform<T>;
    type Error = Error;

    fn node(&self, label: Label) -> Result<&Node<T>> {
        self.nodes
            .iter()
            .find(|node| node.label == label)
            .ok_or_else(|| Error::node(label))
    }

    fn edge(&self, label: Label) -> Result<&Edge<Waveform<T>>> {
        self.edges
            .iter()
            .find(|edge| edge.label == label)
            .ok_or_else(|| Error::edge(label))
    }

    #[trace(channels = [core])]
    fn focus(&mut self, particle: Particulate<T>) -> Label {
        let label = Label(self.particles);
        self.particles += 1;

        let node = Node {
            label,
            particle: particle.clone(),
        };
        self.nodes.insert(node);

        self.refractions.insert(label, label);
        self.world.insert(label, self.worlds);
        self.worlds += 1;
        self.united.insert(label, BTreeSet::from([label]));
        self.future.insert(label, BTreeSet::new());
        self.past.insert(label, BTreeSet::new());

        record::event!(
            channels = [hypergraph],
            label = label.0,
            particle = particle,
            world = self.worlds - 1
        );

        label
    }

    #[trace(channels = [core])]
    fn diffuse(&mut self, signal: Waveform<T>) -> impl Iterator<Item = Label> {
        let labels = (&signal)
            .into_iter()
            .map(|(particle, _)| self.focus(particle.clone()))
            .collect::<Vec<Label>>();

        record::event!(channels = [hypergraph], signal = signal, labels = labels);

        labels.into_iter()
    }

    #[trace(channels = [core])]
    fn absorb(
        &mut self,
        source: BTreeSet<Label>,
        relation: Relation<Waveform<T>>,
    ) -> Result<impl Iterator<Item = Label>> {
        type Matching<V> = (BTreeSet<Label>, BTreeMap<Particulate<V>, usize>);

        fn exclude<P: Clone>(items: &[(P, usize)], index: usize) -> Vec<(P, usize)> {
            items
                .iter()
                .enumerate()
                .filter_map(|(i, (p, c))| (i != index).then_some((p.clone(), *c)))
                .collect()
        }

        fn reduce<P: Clone>(items: &[(P, usize)], index: usize, amount: usize) -> Vec<(P, usize)> {
            items
                .iter()
                .enumerate()
                .filter_map(|(i, (p, c))| {
                    if i == index {
                        (c - amount > 0).then(|| (p.clone(), c - amount))
                    } else {
                        Some((p.clone(), *c))
                    }
                })
                .collect()
        }

        fn enumerate<V: Clone + Eq + Ord + Hash + Serialize + DeserializeOwned>(
            available: Vec<(Label, usize)>,
            needed: Vec<(Particulate<V>, usize)>,
            assigned: BTreeSet<Label>,
            results: &mut Vec<Matching<V>>,
            graph: &Hypergraph<V>,
        ) -> Result<()>
        where
            Particulate<V>: Set + Ranked,
        {
            if needed.is_empty() {
                results.push((assigned, needed.into_iter().collect()));
                return Ok(());
            }

            if available.is_empty() {
                return Ok(());
            }

            for (index, &(label, count)) in available.iter().enumerate() {
                let node = graph.node(label)?;
                let particle = &node.particle;

                for (position, (target, quantity)) in needed.iter().enumerate() {
                    if particle.isomorphic(target).is_none() {
                        continue;
                    }

                    let applied = count.min(*quantity);
                    let world = graph.world.get(&label).ok_or_else(|| Error::world(label))?;

                    let conflicting = assigned
                        .iter()
                        .map(|&assigned_label| graph.world.get(&assigned_label))
                        .collect::<Option<Vec<_>>>()
                        .ok_or_else(|| Error::world(label))?
                        .into_iter()
                        .any(|assigned_world| assigned_world == world);

                    if conflicting {
                        continue;
                    }

                    let mut remaining = exclude(&available, index);
                    if count > applied {
                        remaining.push((label, count - applied));
                    }

                    let reduced = reduce(&needed, position, applied);

                    let mut next = assigned.clone();
                    next.insert(label);

                    enumerate(remaining, reduced, next, results, graph)?;
                }
            }

            Ok(())
        }

        fn search<V: Clone + Eq + Ord + Hash + Serialize + DeserializeOwned>(
            targets: &[(Particulate<V>, usize)],
            index: usize,
            assigned: &mut BTreeSet<Label>,
            unmatched: &mut BTreeMap<Particulate<V>, usize>,
            results: &mut Vec<Matching<V>>,
            graph: &Hypergraph<V>,
        ) where
            Particulate<V>: Set + Ranked,
        {
            if index >= targets.len() {
                results.push((assigned.clone(), unmatched.clone()));
                return;
            }

            let (particle, count) = &targets[index];
            let isomorphic = graph.isomorphics(particle).collect::<Vec<Label>>();

            if isomorphic.is_empty() {
                *unmatched.entry(particle.clone()).or_insert(0) += count;
                search(targets, index + 1, assigned, unmatched, results, graph);
                if let Some(existing) = unmatched.get_mut(particle) {
                    *existing -= count;
                    if *existing == 0 {
                        unmatched.remove(particle);
                    }
                }
                return;
            }

            let mut found = false;
            for label in isomorphic {
                let Some(world) = graph.world.get(&label) else {
                    continue;
                };

                let conflicting = assigned
                    .iter()
                    .filter_map(|&l| graph.world.get(&l))
                    .any(|w| w == world);

                if conflicting {
                    continue;
                }

                found = true;
                assigned.insert(label);
                search(targets, index + 1, assigned, unmatched, results, graph);
                assigned.remove(&label);
            }

            if !found {
                *unmatched.entry(particle.clone()).or_insert(0) += count;
                search(targets, index + 1, assigned, unmatched, results, graph);
                if let Some(existing) = unmatched.get_mut(particle) {
                    *existing -= count;
                    if *existing == 0 {
                        unmatched.remove(particle);
                    }
                }
            }
        }

        let particles = source
            .iter()
            .map(|&label| self.node(label).map(|node| node.particle.clone()))
            .collect::<Result<Vec<_>>>()?;
        let wave = Waveform::from(particles.as_slice());

        let mut all = Vec::new();
        for residual in wave.diverges(&relation.source) {
            let mut sink = relation.sink.particles.clone();
            for (particle, count) in residual.particles {
                *sink.entry(particle).or_insert(0) += count;
            }
            let sink = Waveform::new(sink);

            let targets = (&sink)
                .into_iter()
                .map(|(p, &c)| (p.clone(), c))
                .collect::<Vec<_>>();

            let available = self
                .nodes
                .iter()
                .filter(|node| {
                    targets
                        .iter()
                        .any(|(target, _)| node.particle.isomorphic(target).is_some())
                })
                .map(|node| (node.label, 1))
                .collect::<Vec<_>>();

            let mut matchings = Vec::new();
            enumerate(
                available,
                targets.clone(),
                BTreeSet::new(),
                &mut matchings,
                self,
            )?;

            if matchings.is_empty() {
                search(
                    &targets,
                    0,
                    &mut BTreeSet::new(),
                    &mut BTreeMap::new(),
                    &mut matchings,
                    self,
                );

                if matchings.is_empty() {
                    matchings.push((BTreeSet::new(), targets.into_iter().collect()));
                }
            }

            for (matched, unmatched) in matchings {
                let mut destinations = BTreeSet::new();

                for &label in &matched {
                    if !destinations.insert(label) {
                        return Err(Error::duplicate(label));
                    }
                }

                for (particle, count) in &unmatched {
                    for _ in 0..*count {
                        let label = self.focus(particle.clone());
                        if !destinations.insert(label) {
                            return Err(Error::duplicate(label));
                        }
                    }
                }

                let translation = self.translate(source.clone(), destinations, relation.clone())?;
                if let Some(label) = translation.created() {
                    all.push(label);
                }
            }
        }

        record::event!(
            channels = [hypergraph],
            source = source,
            pattern = relation.source,
            sink = relation.sink,
            residuals = all,
            count = all.len()
        );

        Ok(all.into_iter())
    }

    fn nodes<F>(&self, filter: F) -> impl Iterator<Item = Label> + '_
    where
        F: Fn(&Node<T>) -> bool + 'static,
    {
        self.nodes
            .iter()
            .filter(move |node| filter(node))
            .map(|node| node.label)
    }

    fn edges<F>(&self, filter: F) -> impl Iterator<Item = Label> + '_
    where
        F: Fn(&Edge<Waveform<T>>) -> bool + 'static,
    {
        self.edges
            .iter()
            .filter(move |edge| filter(edge))
            .map(|edge| edge.label)
    }

    fn independent(&self, rank: usize) -> impl Iterator<Item = BTreeSet<Label>> {
        let classes = self
            .united
            .values()
            .map(|set| set.iter().copied().collect::<Vec<Label>>())
            .collect::<Vec<Vec<Label>>>();

        if rank > classes.len() {
            return Vec::new().into_iter();
        }

        classes
            .into_iter()
            .combinations(rank)
            .flat_map(|selected| selected.into_iter().multi_cartesian_product())
            .map(|vec| vec.into_iter().collect::<BTreeSet<Label>>())
            .collect::<Vec<_>>()
            .into_iter()
    }

    fn bipartite(
        &self,
        combination: BTreeSet<Label>,
        rule: &Waveform<T>,
    ) -> Result<impl Iterator<Item = Waveform<T>>> {
        let particles = combination
            .iter()
            .map(|&label| self.node(label).map(|node| node.particle.clone()))
            .collect::<Result<Vec<_>>>()?;

        let wave = Waveform::from(particles.as_slice());
        Ok(bipartitions(&wave, rule).collect::<Vec<_>>().into_iter())
    }

    fn united(&self) -> impl Iterator<Item = impl Iterator<Item = Label> + '_> + '_ {
        self.united.values().map(|set| set.iter().copied())
    }

    fn isomorphics<'a>(&'a self, target: &'a Particulate<T>) -> impl Iterator<Item = Label> + 'a {
        self.nodes
            .iter()
            .filter(move |node| node.particle.isomorphic(target).is_some())
            .map(move |node| node.label)
    }

    #[trace(channels = [core])]
    fn locate(&mut self, label: Label) -> Result<Label> {
        if !self.nodes.iter().any(|node| node.label == label) {
            return Err(Error::node(label));
        }

        if self.refractions.get(&label) == Some(&label) {
            record::event!(channels = [hypergraph], label = label.0, resolved = label.0);
            return Ok(label);
        }

        let past = *self
            .refractions
            .get(&label)
            .ok_or_else(|| Error::refraction(label))?;
        let present = self.locate(past)?;
        self.refractions.insert(label, present);

        record::event!(
            channels = [hypergraph],
            label = label.0,
            resolved = present.0
        );

        Ok(present)
    }

    #[trace(channels = [core])]
    fn unite(&mut self, first: Label, second: Label) -> Result<Label> {
        let anchor = self.locate(first)?;
        let pivot = self.locate(second)?;

        if anchor == pivot {
            record::event!(
                channels = [hypergraph],
                first = first.0,
                second = second.0,
                merged = anchor.0
            );
            return Ok(anchor);
        }
        let foundation = *self
            .world
            .get(&anchor)
            .ok_or_else(|| Error::world(anchor))?;
        let elevation = *self.world.get(&pivot).ok_or_else(|| Error::world(pivot))?;

        let (merged, subset) = match foundation.cmp(&elevation) {
            std::cmp::Ordering::Less => {
                self.refractions.insert(anchor, pivot);
                (pivot, anchor)
            }
            std::cmp::Ordering::Greater => {
                self.refractions.insert(pivot, anchor);
                (anchor, pivot)
            }
            std::cmp::Ordering::Equal => {
                self.refractions.insert(pivot, anchor);
                self.world.insert(anchor, foundation + 1);
                (anchor, pivot)
            }
        };

        if let Some(subset) = self.united.remove(&subset) {
            self.united.entry(merged).or_default().extend(subset);
        }

        record::event!(
            channels = [hypergraph],
            first = first.0,
            second = second.0,
            merged = merged.0,
            subset = subset.0
        );

        Ok(merged)
    }

    #[trace(channels = [core])]
    fn translate(
        &mut self,
        source: BTreeSet<Label>,
        destinations: BTreeSet<Label>,
        rule: Relation<Waveform<T>>,
    ) -> Result<Translation> {
        let existing = self.edges.iter().find(|edge| {
            edge.inference.source == source
                && edge.inference.sink == destinations
                && edge.relation == rule
        });

        if let Some(edge) = existing {
            record::event!(
                channels = [hypergraph],
                source = source,
                destinations = destinations,
                rule = rule,
                edge = edge.label.0,
                state = State::Existing
            );
            return Ok(Translation::Existing(edge.label));
        }

        let label = Label(self.particles);
        self.particles += 1;

        let edge = Edge {
            label,
            inference: component::graph::relation::Edge {
                source: source.clone(),
                sink: destinations.clone(),
            },
            relation: rule.clone(),
        };

        for &origin in &source {
            for &destination in &destinations {
                let _ = self.unite(origin, destination);
            }
        }

        self.edges.insert(edge);

        for &origin in &source {
            self.future.entry(origin).or_default().insert(label);
        }

        for &destination in &destinations {
            self.past.entry(destination).or_default().insert(label);
        }

        record::event!(
            channels = [hypergraph],
            source = source,
            destinations = destinations,
            rule = rule,
            edge = label.0,
            state = State::Created
        );

        Ok(Translation::New(label))
    }

    #[trace(channels = [core])]
    fn infer(&mut self, refractions: Relations<Waveform<T>>) -> Result<Inference> {
        let mut edges = BTreeSet::new();

        for (source, sinks) in &refractions {
            let rank = source.rank();
            let combinations = self.independent(rank).collect::<Vec<_>>();

            for combination in combinations {
                let matchings = self.bipartite(combination.clone(), source)?;
                if matchings.count() == 0 {
                    continue;
                }

                for sink in sinks {
                    let relation = Relation {
                        source: source.clone(),
                        sink: sink.clone(),
                    };
                    for label in self.absorb(combination.clone(), relation)? {
                        edges.insert(label);
                    }
                }
            }
        }

        let snapshot = edges.iter().copied().collect::<Vec<Label>>();
        for &label in &snapshot {
            let edge = self.edge(label)?;
            let sources = edge
                .inference
                .source
                .iter()
                .copied()
                .collect::<Vec<Label>>();
            let relation = edge.relation.clone();

            let ancestral = chains(&sources, &self.past, self);

            for combination in ancestral.into_iter().multi_cartesian_product() {
                let combined = combination.into_iter().collect::<BTreeSet<Label>>();

                let labels = combined.iter().copied().collect::<Vec<_>>();
                let independent = labels.iter().enumerate().all(|(i, &label)| {
                    let class = self
                        .united
                        .values()
                        .find(|members| members.contains(&label));
                    match class {
                        Some(members) => {
                            labels[i + 1..].iter().all(|other| !members.contains(other))
                        }
                        None => true,
                    }
                });

                if !independent {
                    continue;
                }

                let matchings = self.bipartite(combined.clone(), &relation.source)?;
                if matchings.count() == 0 {
                    continue;
                }

                for label in self.absorb(combined.clone(), relation.clone())? {
                    edges.insert(label);
                }
            }
        }

        record::event!(
            channels = [query],
            rules = refractions,
            edges = edges,
            count = edges.len()
        );

        Ok(Inference { edges })
    }

    #[trace(channels = [core])]
    fn fixed(&mut self, refractions: Relations<Waveform<T>>) -> Result<Inference> {
        let mut all = Inference {
            edges: BTreeSet::new(),
        };

        let mut iterations = 0usize;
        loop {
            let inference = self.infer(refractions.clone())?;

            if inference.edges.is_empty() {
                break;
            }

            iterations += 1;
            all.edges.extend(inference.edges);
        }

        record::event!(
            channels = [hypergraph],
            rules = refractions,
            edges = all.edges,
            iterations = iterations,
            count = all.edges.len()
        );

        Ok(all)
    }
}
