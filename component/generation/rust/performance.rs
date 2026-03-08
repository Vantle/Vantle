use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use types::Callable;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Specification {
    pub functions: Vec<Function>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Function {
    pub function: Callable,
    pub select: String,
    pub measure: HashMap<String, Measure>,
    pub sampling: Sampling,
    pub bounds: Vec<Bound>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Measure {
    Length,
    Value,
    Keys,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Sampling {
    pub iterations: usize,
    pub warmup: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bound {
    pub structure: HashMap<String, f64>,
    #[serde(default = "confidence")]
    pub confidence: f64,
}

fn confidence() -> f64 {
    0.833_733_514_515_932
}
