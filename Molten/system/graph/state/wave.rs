use std::hash::Hash;

use observe::trace;
use query::{Polyset, Ranked, Set as QuerySet};
use scale::Scaled;
use serde::Serialize;
use translate::Set;

use component::graph::state::particle::Particle as Particulate;
use component::graph::state::wave::Wave as Waveform;

pub trait Wave: Set + QuerySet + Polyset + Ranked + Scaled {
    type Particle: Set;

    fn particles(&self) -> impl Iterator<Item = (&Self::Particle, &usize)>;
    fn coalesce(particles: &[Self::Particle]) -> Self;
}

impl<T: Clone + Eq + Ord + Hash + Serialize> Wave for Waveform<T> {
    type Particle = Particulate<T>;

    #[trace(channels = [core])]
    fn particles(&self) -> impl Iterator<Item = (&Self::Particle, &usize)> {
        self.into_iter()
    }

    #[trace(channels = [core])]
    fn coalesce(particles: &[Self::Particle]) -> Self {
        Waveform::from(particles)
    }
}

#[trace(channels = [core])]
pub fn matchings<W>(source: &W, sink: &W) -> impl Iterator<Item = W>
where
    W: Wave + Clone + Eq + Ord + Serialize,
    W::Particle: Clone + QuerySet + Ranked,
{
    let mut results = Vec::new();

    fn exclude<P: Clone>(source: &[(P, usize)], index: usize) -> Vec<(P, usize)> {
        source
            .iter()
            .enumerate()
            .filter_map(|(i, (p, c))| (i != index).then_some((p.clone(), *c)))
            .collect()
    }

    fn reduce<P: Clone>(remaining: &[(P, usize)], target: usize, amount: usize) -> Vec<(P, usize)> {
        remaining
            .iter()
            .enumerate()
            .filter_map(|(i, (p, c))| {
                if i == target {
                    let count = c - amount;
                    (count > 0).then(|| (p.clone(), count))
                } else {
                    Some((p.clone(), *c))
                }
            })
            .collect()
    }

    fn enumerate<W>(
        current: usize,
        source: Vec<(W::Particle, usize)>,
        remaining: Vec<(W::Particle, usize)>,
        residue: Vec<(W::Particle, usize)>,
        generated: &mut Vec<W>,
    ) where
        W: Wave + Clone,
        W::Particle: Clone + QuerySet + Ranked,
    {
        if remaining.is_empty() {
            let generation = residue
                .iter()
                .chain(&source)
                .flat_map(|(p, count)| std::iter::repeat_n(p.clone(), *count))
                .collect::<Vec<_>>();
            generated.push(W::coalesce(&generation));
            return;
        }

        let maximum = source.iter().map(|(p, _)| p.rank()).max().unwrap_or(0);

        if current > maximum {
            return;
        }

        let candidates = source
            .iter()
            .enumerate()
            .filter(|(_, (p, _))| p.rank() == current)
            .collect::<Vec<_>>();

        for (index, (particle, count)) in &candidates {
            for (position, (target, quantity)) in remaining.iter().enumerate() {
                if particle.superset(target).is_none() {
                    continue;
                }

                let applied = (*count).min(*quantity);

                let mut source = exclude(&source, *index);
                if *count > applied {
                    source.push((particle.clone(), *count - applied));
                }

                let remaining = reduce(&remaining, position, applied);

                let mut residue = residue.clone();
                if let Some(residual) = particle.diverge(target) {
                    residue.push((residual, applied));
                }

                enumerate(0, source, remaining, residue, generated);
            }

            let source = exclude(&source, *index);
            let mut residue = residue.clone();
            residue.push((particle.clone(), *count));

            enumerate(current + 1, source, remaining.clone(), residue, generated);
        }

        enumerate(current + 1, source, remaining, residue, generated);
    }

    let sources = source
        .particles()
        .map(|(p, &c)| (p.clone(), c))
        .collect::<Vec<_>>();
    let sinks = sink
        .particles()
        .map(|(p, &c)| (p.clone(), c))
        .collect::<Vec<_>>();

    enumerate(0, sources, sinks, Vec::new(), &mut results);

    record::event!(
        channels = [matching],
        source = source,
        sink = sink,
        matchings = results,
        count = results.len()
    );

    results.into_iter()
}
