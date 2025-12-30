use itertools::Itertools;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};
use std::hash::Hash;

#[serde_as]
#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(transparent)]
#[serde(bound(serialize = "T: Serialize", deserialize = "T: DeserializeOwned"))]
pub struct Particle<T: Eq + Ord> {
    #[serde_as(as = "Vec<(_, _)>")]
    pub elements: BTreeMap<T, usize>,
}

impl<T: Clone + Eq + Ord> Particle<T> {
    #[must_use]
    pub fn fundamental(data: T) -> Self {
        Particle {
            elements: BTreeMap::from_iter([(data, 1)]),
        }
    }
}

impl<T: Eq + Ord> Particle<T> {
    #[must_use]
    pub fn new(elements: BTreeMap<T, usize>) -> Self {
        Particle { elements }
    }

    pub fn iter(&self) -> std::collections::btree_map::Iter<'_, T, usize> {
        self.elements.iter()
    }
}

impl<T: Clone + Eq + Ord + Hash> Hash for Particle<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        for (label, count) in &self.elements {
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
        Particle::new(
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

impl<'a, T: Eq + Ord> IntoIterator for &'a Particle<T> {
    type Item = (&'a T, &'a usize);
    type IntoIter = std::collections::btree_map::Iter<'a, T, usize>;

    fn into_iter(self) -> Self::IntoIter {
        self.elements.iter()
    }
}

impl<T: Eq + Ord> IntoIterator for Particle<T> {
    type Item = (T, usize);
    type IntoIter = std::collections::btree_map::IntoIter<T, usize>;

    fn into_iter(self) -> Self::IntoIter {
        self.elements.into_iter()
    }
}

impl Display for Particle<String> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let formatted = self
            .elements
            .iter()
            .map(|(element, count)| {
                if *count == 1 {
                    element.clone()
                } else {
                    format!("{element} × {count}")
                }
            })
            .collect::<Vec<_>>()
            .join(" · ");
        write!(f, "{formatted}")
    }
}
