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

pub use error;

#[serde_as]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Valued<Value: Eq + Hash> {
    #[serde_as(as = "Vec<(_, _)>")]
    indices: HashMap<Arc<Value>, usize>,
    values: HashMap<usize, Arc<Value>>,
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

    /// Iterate over all stored values together with their numeric aliases.
    pub fn iter(&self) -> impl Iterator<Item = (usize, &Value)> {
        self.values
            .iter()
            .map(|(index, value)| (*index, value.as_ref()))
    }
}

impl<Value: Eq + Hash + Debug> Aliased for Valued<Value> {
    type Value = Value;
    type Identity = usize;
    type Error = error::Error;

    fn value(&self, alias: Self::Identity) -> Result<&Self::Value, Self::Error> {
        self.values
            .get(&alias)
            .map(|value| value.as_ref())
            .ok_or_else(|| error::Missing::element(alias).into())
    }

    fn alias(&self, value: &Self::Value) -> Result<Self::Identity, Self::Error> {
        self.indices
            .iter()
            .find(|(reference, _)| reference.as_ref() == value)
            .map(|(_, index)| index)
            .cloned()
            .ok_or_else(|| error::Missing::element(value).into())
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
            if index == usize::MAX {
                let error = error::allocation::Allocation::Limit;
                panic!("{}", error);
            }

            if let Some(existing) = indices.get::<Arc<Template>>(attribute.borrow()) {
                debug!(
                    "Unified constraint {:?} on allocation {:?}.",
                    attribute, existing
                );
                continue;
            }

            if let Some(existing) = indices.insert(attribute.clone(), index) {
                let error = error::allocation::Allocation::unification(format!(
                    "constraint {:?} on allocation {:?} after attempted merge",
                    attribute, existing
                ));
                panic!("{}", error);
            }

            if attributes.insert(index, attribute.clone()).is_some() {
                let error = error::allocation::Allocation::collision(index);
                panic!("{}", error);
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
