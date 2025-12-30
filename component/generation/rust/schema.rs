use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use types::Callable;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(bound(deserialize = "T: serde::de::DeserializeOwned + Default"))]
pub struct Cases<T = Value> {
    pub functions: Vec<Function<T>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(bound(deserialize = "T: serde::de::DeserializeOwned + Default"))]
pub struct Function<T = Value> {
    pub function: Callable,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub parameters: HashMap<String, T>,
    #[serde(default)]
    pub returns: HashMap<String, T>,
    pub cases: Vec<Case<T>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(bound(deserialize = "T: serde::de::DeserializeOwned + Default"))]
pub struct Case<T = Value> {
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub parameters: HashMap<String, T>,
    #[serde(default)]
    pub returns: HashMap<String, T>,
}
