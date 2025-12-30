use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};
use std::hash::Hash;

use itertools::Itertools;
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use serde_with::serde_as;

pub use particle;

use particle::Particle;

#[serde_as]
#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(transparent)]
#[serde(bound(serialize = "T: Serialize", deserialize = "T: DeserializeOwned"))]
pub struct Wave<T: Eq + Ord> {
    #[serde_as(as = "Vec<(_, _)>")]
    pub particles: BTreeMap<Particle<T>, usize>,
}

impl<T: Clone + Eq + Ord> Wave<T> {
    #[must_use]
    pub fn monochromatic(data: Particle<T>) -> Self {
        Wave {
            particles: BTreeMap::from_iter([(data, 1)]),
        }
    }

    #[must_use]
    pub fn polychromatic(data: Particle<T>, multiplicity: usize) -> Self {
        Wave {
            particles: BTreeMap::from_iter([(data, multiplicity)]),
        }
    }
}

impl<T: Eq + Ord> Wave<T> {
    #[must_use]
    pub fn new(particles: BTreeMap<Particle<T>, usize>) -> Self {
        Wave { particles }
    }

    #[must_use]
    pub fn empty(&self) -> bool {
        self.particles.is_empty()
    }

    pub fn iter(&self) -> std::collections::btree_map::Iter<'_, Particle<T>, usize> {
        self.particles.iter()
    }
}

impl<T: Clone + Eq + Ord + Hash> Hash for Wave<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        for (particle, count) in &self.particles {
            particle.hash(state);
            count.hash(state);
        }
    }
}

impl<'a, T: Eq + Ord> IntoIterator for &'a Wave<T> {
    type Item = (&'a Particle<T>, &'a usize);
    type IntoIter = std::collections::btree_map::Iter<'a, Particle<T>, usize>;

    fn into_iter(self) -> Self::IntoIter {
        self.particles.iter()
    }
}

impl<T: Clone + Eq + Ord + Hash> From<&[Particle<T>]> for Wave<T> {
    fn from(elements: &[Particle<T>]) -> Self {
        Wave::new(
            elements
                .iter()
                .cloned()
                .counts()
                .into_iter()
                .collect::<BTreeMap<_, _>>(),
        )
    }
}

impl Display for Wave<String> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let formatted = self
            .particles
            .iter()
            .map(|(particle, count)| {
                if *count == 1 {
                    particle.to_string()
                } else {
                    format!("{particle} × {count}")
                }
            })
            .collect::<Vec<_>>()
            .join(", ");
        write!(f, "{formatted}")
    }
}
