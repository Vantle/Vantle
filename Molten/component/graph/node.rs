use itertools::Itertools;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::hash::Hash;
use std::hash::Hasher;

use traits::node::{Queryable, Scaled, Translatable};

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Node<T: Eq + Ord>(BTreeMap<T, usize>);

impl<T: Eq + Ord + Hash> Hash for Node<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.iter().fold(0usize, |hash, (label, count)| {
            label.hash(state);
            count.hash(state);
            hash
        });
    }
}

impl<T, U> From<&[U]> for Node<T>
where
    T: Eq + Ord + Hash + From<U>,
    U: Clone,
{
    fn from(elements: &[U]) -> Self {
        Node(
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

impl<T: Clone + Eq + Ord> Scaled for Node<T> {
    type Magnitude = T;

    fn scale(&self, basis: &Self::Magnitude) -> Self::Magnitude {
        basis.clone()
    }
}

impl<T: Clone + Eq + Ord> Queryable for Node<T> {
    fn subset(&self, basis: &Self) -> Option<&Self> {
        self.0
            .iter()
            .all(|(key, &count)| {
                basis
                    .0
                    .get(key)
                    .is_some_and(|&basis_count| count <= basis_count)
            })
            .then_some(self)
    }

    fn superset(&self, basis: &Self) -> Option<&Self> {
        basis
            .0
            .iter()
            .all(|(key, &count)| {
                self.0
                    .get(key)
                    .is_some_and(|&self_count| self_count >= count)
            })
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

impl<T: Clone + Eq + Ord> Translatable for Node<T> {
    fn join(&self, basis: &Self) -> Option<Self> {
        let mut result = self.0.clone();
        for (key, value) in &basis.0 {
            *result.entry(key.clone()).or_insert(0) += value;
        }
        Some(Node(result))
    }

    fn intersect(&self, basis: &Self) -> Option<Self> {
        let intersection: BTreeMap<T, usize> = self
            .0
            .iter()
            .filter_map(|(key, &count)| {
                basis
                    .0
                    .get(key)
                    .map(|&basis_count| (key.clone(), count.min(basis_count)))
            })
            .filter(|(_, count)| *count > 0)
            .collect();

        (!intersection.is_empty()).then_some(Node(intersection))
    }

    fn diverge(&self, basis: &Self) -> Option<Self> {
        let difference: BTreeMap<T, usize> = self
            .0
            .iter()
            .filter_map(|(key, &count)| match basis.0.get(key) {
                Some(&basis_count) if count > basis_count => {
                    Some((key.clone(), count - basis_count))
                }
                None => Some((key.clone(), count)),
                _ => None,
            })
            .collect();

        (!difference.is_empty()).then_some(Node(difference))
    }
}
