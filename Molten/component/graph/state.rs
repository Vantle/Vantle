use states::Stateful;

use itertools::Itertools;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::hash::Hash;
use std::hash::Hasher;

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct State<T: Eq + Ord>(BTreeMap<T, usize>);

impl<T: Eq + Ord + Hash> Hash for State<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.iter().fold(0usize, |hash, (label, count)| {
            label.hash(state);
            count.hash(state);
            hash
        });
    }
}

impl<T: Eq + Ord + Hash + Clone> From<&[T]> for State<T> {
    fn from(elements: &[T]) -> Self {
        State(
            elements
                .iter()
                .cloned()
                .counts()
                .into_iter()
                .collect::<BTreeMap<_, _>>(),
        )
    }
}

impl<T: Clone + Eq + Ord> Stateful<T> for State<T> {
    fn scale(&self, basis: &T) -> Option<usize> {
        self.0.get(basis).cloned()
    }

    fn product(&self, _beta: &Self) -> Self {
        // let mut product = self.0.clone();
        // for (key, value) in &beta.0 {
        //     *product.entry(key.clone()).or_insert(0) += value;
        // }
        // State(product)
        todo!()
    }

    fn union(&self, _beta: &Self) -> Option<&Self> {
        // let union: BTreeMap<T, usize> = self.0.iter()
        //     .filter_map(|(key, alpha)| {
        //         beta.0.get(key).map(|beta| (key.clone(), alpha + beta))
        //     })
        //     .collect();
        todo!()
    }

    fn intersection(&self, _test: &Self) -> Option<&Self> {
        todo!()
    }

    fn elimination(&self, _test: &Self) -> Option<&Self> {
        todo!()
    }

    fn divergence(&self, _test: &Self) -> Option<&Self> {
        todo!()
    }

    fn subset(&self, _test: &Self) -> Option<&Self> {
        todo!()
    }

    fn superset(&self, _test: &Self) -> Option<&Self> {
        todo!()
    }

    fn disjoint(&self, _test: &Self) -> Option<&Self> {
        todo!()
    }

    fn equivalence(&self, _test: &Self) -> Option<&Self> {
        todo!()
    }
}

//     pub fn filter<F>(&self, filter: &State<T>, condition: F) -> Option<&Self>
//     where
//         F: Fn((&T, &usize), (&T, &usize)) -> bool,
//     {
//         (self.0.iter().all(|(label, &quantity)| {
//             filter
//                 .0
//                 .get(label)
//                 .is_some_and(|&comparison| condition((label, &quantity), (label, &comparison)))
//         }))
//         .then_some(self)
//     }

//     pub fn equivalence(&self, filter: &State<T>) -> Option<&Self> {
//         self.filter(filter, |(_, &alpha), (_, &beta)| alpha == beta)
//     }

//     pub fn superset(&self, filter: &State<T>) -> Option<&Self> {
//         self.filter(filter, |(_, &alpha), (_, &beta)| alpha >= beta)
//     }

//     pub fn subset(&self, filter: &State<T>) -> Option<&Self> {
//         self.filter(filter, |(_, &alpha), (_, &beta)| alpha <= beta)
//     }
// }

// impl<T: Default + Hash + Eq + Clone + PartialEq> Hash for State<T> {
//     todo!()
// }

// impl<T: Default + Hash + Eq + PartialEq + Clone + Serialize> Stateful<T> for State<T> {
//     fn scale(&self, basis: &T) -> usize {
//         *self.0.get(basis).unwrap_or(&0)
//     }
// }
