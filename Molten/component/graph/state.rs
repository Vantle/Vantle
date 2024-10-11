use states::Stateful;

use itertools::Itertools;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use std::collections::HashMap;
use std::hash::DefaultHasher;
use std::hash::{Hash, Hasher};

#[serde_as]
#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct State<T: Hash + Eq + PartialEq + Clone + Serialize + for<'a> Deserialize<'a>>(
    #[serde_as(as = "Vec<(_, _)>")] HashMap<T, usize>,
);

impl<T: Hash + Eq + PartialEq + Clone + Serialize + for<'a> Deserialize<'a>> State<T> {
    pub fn new(elements: &[T]) -> Self {
        let mut state = HashMap::new();
        for element in elements.iter() {
            *state.entry(element.clone()).or_insert(0) += 1;
        }
        Self(state)
    }

    pub fn filter<F>(&self, filter: &State<T>, condition: F) -> Option<&Self>
    where
        F: Fn((&T, &usize), (&T, &usize)) -> bool,
    {
        (self.0.iter().all(|(label, &quantity)| {
            filter
                .0
                .get(label)
                .is_some_and(|&comparison| condition((label, &quantity), (label, &comparison)))
        }))
        .then_some(self)
    }

    pub fn equivalence(&self, filter: &State<T>) -> Option<&Self> {
        self.filter(filter, |(_, &alpha), (_, &beta)| alpha == beta)
    }

    pub fn superset(&self, filter: &State<T>) -> Option<&Self> {
        self.filter(filter, |(_, &alpha), (_, &beta)| alpha >= beta)
    }

    pub fn subset(&self, filter: &State<T>) -> Option<&Self> {
        self.filter(filter, |(_, &alpha), (_, &beta)| alpha <= beta)
    }
}

impl<T: Hash + Eq + Clone + Serialize + for<'a> Deserialize<'a>> Hash for State<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for (key, value) in self.0.iter().sorted_by_key(|(k, _)| {
            let mut hasher = DefaultHasher::new();
            k.hash(&mut hasher);
            hasher.finish()
        }) {
            key.hash(state);
            value.hash(state);
        }
    }
}

impl<T: Hash + Eq + Clone + Serialize + for<'a> Deserialize<'a>> Stateful<T> for State<T> {
    fn scale(&self, basis: &T) -> usize {
        *self.0.get(basis).unwrap_or(&0)
    }
}

pub struct States<T: Hash + Eq + Clone + Serialize + for<'a> Deserialize<'a>>(Vec<State<T>>);

impl<T: Hash + Eq + Clone + Serialize + for<'a> Deserialize<'a>> States<T> {
    pub fn initialialize(states: &[State<T>]) -> Self {
        States(states.to_vec())
    }

    pub fn translate(&mut self, target: &State<T>, replacements: Vec<State<T>>) {
        if let Some(position) = self.0.iter().position(|state| state == target) {
            self.0.splice(position..=position, replacements);
        }
    }

    pub fn filter<F>(&self, filter: &State<T>, condition: F) -> Vec<&State<T>>
    where
        F: Fn((&T, &usize), (&T, &usize)) -> bool,
    {
        self.0
            .iter()
            .filter_map(|state| state.filter(filter, &condition))
            .collect()
    }

    pub fn equivalence(&self, filter: &State<T>) -> Vec<&State<T>> {
        self.filter(filter, |(_, &alpha), (_, &beta)| alpha == beta)
    }

    pub fn superset(&self, filter: &State<T>) -> Vec<&State<T>> {
        self.filter(filter, |(_, &alpha), (_, &beta)| alpha >= beta)
    }

    pub fn subset(&self, filter: &State<T>) -> Vec<&State<T>> {
        self.filter(filter, |(_, &alpha), (_, &beta)| alpha <= beta)
    }
}
