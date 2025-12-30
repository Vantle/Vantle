use std::{collections::HashMap, hash::Hash, sync::Arc};

use serde::{Deserialize, Serialize};
use serde_with::serde_as;

#[serde_as]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Valued<Value: Eq + Hash> {
    #[serde_as(as = "Vec<(_, _)>")]
    pub indices: HashMap<Arc<Value>, usize>,
    pub values: HashMap<usize, Arc<Value>>,
    pub counter: usize,
}

impl<Value: Eq + Hash> Valued<Value> {
    #[must_use]
    pub fn new(
        indices: HashMap<Arc<Value>, usize>,
        values: HashMap<usize, Arc<Value>>,
        counter: usize,
    ) -> Self {
        Self {
            indices,
            values,
            counter,
        }
    }
}

impl<Value: Eq + Hash> Default for Valued<Value> {
    fn default() -> Self {
        Self {
            indices: HashMap::new(),
            values: HashMap::new(),
            counter: 0,
        }
    }
}
