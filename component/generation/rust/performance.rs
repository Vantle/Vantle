use std::collections::HashMap;
use std::time::Duration;

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
#[serde(untagged)]
pub enum Bound {
    At {
        at: HashMap<String, usize>,
        #[serde(with = "duration")]
        within: Duration,
    },
    Determination {
        determination: f64,
    },
}

mod duration {
    use super::{format, parse};
    use serde::{Deserialize, Serializer};
    use std::time::Duration;

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Duration, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let string = String::deserialize(deserializer)?;
        parse(&string)
            .ok_or_else(|| serde::de::Error::custom(std::format!("invalid duration: {string}")))
    }

    pub fn serialize<S>(duration: &Duration, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&format(*duration))
    }
}

fn parse(input: &str) -> Option<Duration> {
    let input = input.trim();
    if let Some(rest) = input.strip_suffix("ms") {
        rest.trim()
            .parse::<f64>()
            .ok()
            .map(Duration::from_secs_f64)
            .map(|d| d / 1000)
    } else if let Some(rest) = input.strip_suffix('s') {
        rest.trim().parse::<f64>().ok().map(Duration::from_secs_f64)
    } else if let Some(rest) = input.strip_suffix('m') {
        rest.trim()
            .parse::<f64>()
            .ok()
            .map(|v| Duration::from_secs_f64(v * 60.0))
    } else {
        input.parse::<f64>().ok().map(Duration::from_secs_f64)
    }
}

#[must_use]
pub fn format(duration: Duration) -> String {
    let seconds = duration.as_secs_f64();
    if seconds >= 1.0 {
        format!("{seconds:.1}s")
    } else if seconds >= 0.001 {
        format!("{:.1}ms", seconds * 1000.0)
    } else {
        format!("{:.1}µs", seconds * 1_000_000.0)
    }
}
