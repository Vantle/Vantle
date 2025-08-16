//! Core types for the generator component.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use syn::{Signature, Type};

/// A reference to a function, with its module path and name parsed.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Callable {
    /// The full path as originally specified (e.g., "math::operators::multiply")
    pub qualified: String,
    /// The module path (e.g., "math::operators")
    pub module: String,
    /// The function name (e.g., "multiply")
    pub name: String,
}

impl Callable {
    /// Create a new [`Callable`] from a fully-qualified path.
    pub fn new(input: String) -> Self {
        // Convert the dot-separated path (`a.b.c`) into the canonical Rust
        // path (`a::b::c`).
        let qualified = input.split('.').collect::<Vec<_>>().join("::");

        // Determine the module path and the function name using the (now
        // canonical) `qualified` string.
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

// Generation-specific types
pub type Functions = HashMap<String, Signature>;
pub type Structs = HashMap<String, HashMap<String, Type>>; // struct_name -> field_name -> field_type
pub type Counters = HashMap<String, usize>;
