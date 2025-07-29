//! Core types for the generator component.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use syn::{Signature, Type};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Callable {
    pub qualified: String,
    pub module: String,
    pub name: String,
}

impl Callable {
    pub fn new(input: String) -> Self {
        let qualified = input.split('.').collect::<Vec<_>>().join("::");

        let (module, name) = match qualified.rfind("::") {
            Some(pos) => (
                qualified[..pos].to_string(),
                qualified[pos + 2..].to_string(),
            ),
            None => (String::new(), qualified.clone()),
        };

        Self {
            qualified,
            module,
            name,
        }
    }
}

impl<'de> Deserialize<'de> for Callable {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(Callable::new(String::deserialize(deserializer)?))
    }
}

impl Serialize for Callable {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.qualified.serialize(serializer)
    }
}

pub type Functions = HashMap<String, Signature>;
pub type Structs = HashMap<String, HashMap<String, Type>>;
pub type Counters = HashMap<String, usize>;
