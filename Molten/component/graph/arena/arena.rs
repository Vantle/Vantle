use std::{
    borrow::Borrow,
    collections::{HashMap, VecDeque},
    fmt::Debug,
    hash::Hash,
    sync::Arc,
};

pub use error;
use log::debug;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use traits::{attribute::Contextualized, node};

#[serde_as]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Valued<Value: Eq + Hash> {
    #[serde_as(as = "Vec<(_, _)>")]
    indices: HashMap<Arc<Value>, usize>,
    values: HashMap<usize, Arc<Value>>,
    counter: usize,
}

impl<Value: Eq + Hash + Debug> Valued<Value> {
    pub fn new(indices: HashMap<Arc<Value>, usize>, values: HashMap<usize, Arc<Value>>) -> Self {
        let counter = values.keys().max().map(|&max| max + 1).unwrap_or(0);
        Self {
            indices,
            values,
            counter,
        }
    }

    pub fn none() -> Self {
        Self {
            indices: HashMap::new(),
            values: HashMap::new(),
            counter: 0,
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = (usize, &Value)> {
        self.values
            .iter()
            .map(|(index, value)| (*index, value.as_ref()))
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
}

impl<Value: Eq + Hash + Debug> Valued<Value> {
    pub fn insert(&mut self, value: Value) -> Result<usize, error::allocation::Allocation>
    where
        Value: Contextualized<Context = Value> + Clone,
    {
        let reference = Arc::new(value);

        if let Some(&existing) = self.indices.get(&reference) {
            debug!(
                "Unified constraint {:?} on allocation {:?}.",
                reference, existing
            );
            return Ok(existing);
        }

        if self.counter == usize::MAX {
            return Err(error::allocation::Allocation::Limit);
        }

        let index = self.counter;
        self.counter += 1;

        self.indices.insert(reference.clone(), index);
        self.values.insert(index, reference.clone());

        let mut queue = VecDeque::new();
        for context in reference.context() {
            queue.push_back(Arc::from(context.clone()));
        }

        while let Some(attribute) = queue.pop_back() {
            if let Some(existing) = self.indices.get::<Arc<Value>>(attribute.borrow()) {
                debug!(
                    "Unified constraint {:?} on allocation {:?}.",
                    attribute, existing
                );
                continue;
            }

            if self.counter == usize::MAX {
                return Err(error::allocation::Allocation::Limit);
            }

            let context_index = self.counter;
            self.counter += 1;

            self.indices.insert(attribute.clone(), context_index);
            self.values.insert(context_index, attribute.clone());

            for context in attribute.context() {
                queue.push_back(Arc::from(context.clone()));
            }
        }

        Ok(index)
    }
}

impl<Template> From<Template> for Valued<Template>
where
    Template: Contextualized<Context = Template> + Hash + Eq + Clone + Debug,
{
    fn from(source: Template) -> Self {
        let mut arena = Self::none();
        arena.insert(source).expect("Failed to build initial arena");
        arena
    }
}

impl<Value: Eq + Hash + Debug> node::Valued for Valued<Value> {
    type Value = Value;
    type Identity = usize;
    type Error = error::Error;

    fn value(&self, alias: Self::Identity) -> Result<&Self::Value, Self::Error> {
        self.values
            .get(&alias)
            .map(|value| value.as_ref())
            .ok_or_else(|| error::Missing::element(alias).into())
    }
}

impl<Value: Eq + Hash + Debug> node::Aliased for Valued<Value> {
    type Value = Value;
    type Identity = usize;
    type Error = error::Error;

    fn alias(&self, value: &Self::Value) -> Result<Self::Identity, Self::Error> {
        self.indices
            .iter()
            .find(|(reference, _)| reference.as_ref() == value)
            .map(|(_, index)| index)
            .cloned()
            .ok_or_else(|| error::Missing::element(value).into())
    }
}
