use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Observation {
    pub point: Vec<f64>,
    pub mean: f64,
    pub deviation: f64,
    pub count: usize,
}

#[derive(Debug, Clone)]
pub struct Fit {
    pub terms: Vec<Term>,
    pub normalization: f64,
}

#[derive(Debug, Clone)]
pub struct Term {
    pub exponent: Vec<usize>,
    pub coefficient: f64,
}

#[derive(Debug, Clone)]
pub struct Predicted {
    pub point: Vec<f64>,
    pub mean: f64,
    pub deviation: f64,
    pub count: usize,
    pub predicted: f64,
    pub interval: [f64; 2],
}
