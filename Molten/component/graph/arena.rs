use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use std::hash::Hash;

use traits::attribute::Contextualized;
use traits::node::Aliased;

use log::debug;

use std::borrow::Borrow;

use std::fmt::Debug;
use std::sync::Arc;

use serde_with::serde_as;
use thiserror::Error;

#[serde_as]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Valued<Value: Eq + Hash> {
    #[serde_as(as = "Vec<(_, _)>")]
    indices: HashMap<Arc<Value>, usize>,
    values: HashMap<usize, Arc<Value>>,
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("Arena missing element: {0}")]
    Missing(String),
}

impl<Value: Eq + Hash> Valued<Value> {
    pub fn new(indices: HashMap<Arc<Value>, usize>, values: HashMap<usize, Arc<Value>>) -> Self {
        Self { indices, values }
    }

    pub fn none() -> Self {
        Self {
            indices: HashMap::new(),
            values: HashMap::new(),
        }
    }
}

impl<Value: Eq + Hash + Debug> Aliased for Valued<Value> {
    type Value = Value;
    type Identity = usize;
    type Error = Error;

    fn value(&self, alias: Self::Identity) -> Result<&Self::Value, Self::Error> {
        self.values
            .get(&alias)
            .map(|value| value.as_ref())
            .ok_or(Error::Missing(alias.to_string()))
    }

    fn alias(&self, value: &Self::Value) -> Result<Self::Identity, Self::Error> {
        self.indices
            .iter()
            .find(|(reference, _)| reference.as_ref() == value)
            .map(|(_, index)| index)
            .cloned()
            .ok_or(Error::Missing(format!("{:#?}", value)))
    }
}

impl<Template> From<Template> for Valued<Template>
where
    Template: Contextualized<Context = Template> + Hash + Eq + Clone + Debug,
{
    fn from(source: Template) -> Self {
        let mut indices = HashMap::<Arc<Template>, usize>::new();
        let mut attributes = HashMap::<usize, Arc<Template>>::new();
        let mut queue = VecDeque::from(vec![Arc::from(source)]);
        let mut index = 0usize;

        while let Some(attribute) = queue.pop_back() {
            assert!(index < usize::MAX, "Computation limit for Arena reached");

            if let Some(existing) = indices.get::<Arc<Template>>(attribute.borrow()) {
                debug!(
                    "Unified constraint {:?} on allocation {:?}.",
                    attribute, existing
                );
                continue;
            }

            if let Some(existing) = indices.insert(attribute.clone(), index) {
                panic!(
                    "Unified constraint {:?} on allocation {:?} after attempted merge",
                    attribute, existing
                )
            }

            if let Some(existing) = attributes.insert(index, attribute.clone()) {
                panic!("Collision occurred during Arena allocation {:?} when index {:?} should have been free", existing, index);
            }
            index += 1;

            for context in attribute.context() {
                queue.push_back(Arc::from(context.clone()));
            }
        }

        Self {
            indices,
            values: attributes,
        }
    }
}
