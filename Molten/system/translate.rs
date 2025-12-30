use std::collections::BTreeMap;

use observe::trace;

use component::graph::state::particle::Particle;
use component::graph::state::wave::Wave;

pub trait Set
where
    Self: Sized,
{
    fn join(&self, basis: &Self) -> Option<Self>;
    fn intersect(&self, basis: &Self) -> Option<Self>;
    fn diverge(&self, basis: &Self) -> Option<Self>;
}

impl<T: Clone + Eq + Ord> Set for Particle<T> {
    #[trace(channels = [core])]
    fn join(&self, basis: &Self) -> Option<Self> {
        let mut modified = false;
        let mut result = self.elements.clone();

        for (key, &count) in &basis.elements {
            if count > 0 {
                modified = true;
            }
            *result.entry(key.clone()).or_insert(0) += count;
        }

        if !modified {
            return None;
        }

        Some(Particle::new(result))
    }

    #[trace(channels = [core])]
    fn intersect(&self, basis: &Self) -> Option<Self> {
        let result = self
            .elements
            .iter()
            .filter_map(|(key, &count)| {
                basis
                    .elements
                    .get(key)
                    .map(|&relative| (key.clone(), count.min(relative)))
            })
            .collect::<BTreeMap<T, usize>>();

        (!result.is_empty()).then_some(Particle::new(result))
    }

    #[trace(channels = [core])]
    fn diverge(&self, basis: &Self) -> Option<Self> {
        let difference = self
            .elements
            .iter()
            .filter_map(|(key, &count)| match basis.elements.get(key) {
                Some(&relative) if count > relative => Some((key.clone(), count - relative)),
                None => Some((key.clone(), count)),
                _ => None,
            })
            .collect::<BTreeMap<T, usize>>();

        (!difference.is_empty()).then_some(Particle::new(difference))
    }
}

impl<T: Clone + Eq + Ord> Set for Wave<T> {
    #[trace(channels = [core])]
    fn join(&self, basis: &Self) -> Option<Self> {
        let mut modified = false;
        let mut result = self.particles.clone();

        for (particle, &count) in &basis.particles {
            if count > 0 {
                modified = true;
            }
            *result.entry(particle.clone()).or_insert(0) += count;
        }

        if !modified {
            return None;
        }

        Some(Wave::new(result))
    }

    #[trace(channels = [core])]
    fn intersect(&self, basis: &Self) -> Option<Self> {
        let mut result: BTreeMap<Particle<T>, usize> = BTreeMap::new();

        for (left, &lefts) in &self.particles {
            for (right, &rights) in &basis.particles {
                if let Some(common) = left.intersect(right) {
                    let count = lefts.min(rights);
                    if count > 0 {
                        *result.entry(common).or_insert(0) += count;
                    }
                }
            }
        }

        (!result.is_empty()).then_some(Wave::new(result))
    }

    #[trace(channels = [core])]
    fn diverge(&self, basis: &Self) -> Option<Self> {
        let difference = self
            .particles
            .iter()
            .filter_map(|(particle, &count)| match basis.particles.get(particle) {
                Some(&relative) if count > relative => Some((particle.clone(), count - relative)),
                None => Some((particle.clone(), count)),
                _ => None,
            })
            .collect::<BTreeMap<Particle<T>, usize>>();

        (!difference.is_empty()).then_some(Wave::new(difference))
    }
}
