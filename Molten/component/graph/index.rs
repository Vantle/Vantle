use std::fmt::Debug;
use std::hash::Hash;

use arena::Valued;
use attribute::{Attribute, Value};
use relation::Related;
use serde::{Deserialize, Serialize};
use state::wave::Wave;

#[derive(Serialize, Deserialize)]
#[serde(bound(serialize = "T: Value", deserialize = "T: Value"))]
pub struct Index<T>
where
    T: Eq + Hash + Debug,
{
    pub arena: Valued<Attribute<T>>,
    pub relations: Related<Wave<usize>>,
}

impl<T> Default for Index<T>
where
    T: Value,
{
    fn default() -> Self {
        Self {
            arena: Valued::default(),
            relations: Related::default(),
        }
    }
}
