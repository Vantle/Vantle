use std::collections::HashMap;

use miette::Diagnostic;
use thiserror::Error;

#[derive(Debug, Clone)]
pub struct Expression {
    terms: Vec<(Vec<usize>, f64)>,
}

impl Expression {
    #[must_use]
    pub fn new(terms: Vec<(Vec<usize>, f64)>) -> Self {
        Self { terms }
    }

    pub fn parse(map: &HashMap<String, f64>) -> Result<Self, Error> {
        let mut terms = Vec::with_capacity(map.len());
        for (key, &coefficient) in map {
            let exponent = parse(key)?;
            terms.push((exponent, coefficient));
        }
        Ok(Self { terms })
    }

    #[must_use]
    pub fn evaluate(&self, point: &[f64]) -> f64 {
        self.terms
            .iter()
            .map(|(exponent, coefficient)| {
                let monomial: f64 = exponent
                    .iter()
                    .zip(point.iter())
                    .map(|(&power, &base)| {
                        let exponent = i32::try_from(power).unwrap_or(i32::MAX);
                        base.powi(exponent)
                    })
                    .product();
                coefficient * monomial
            })
            .sum()
    }

    #[must_use]
    pub fn representation(&self) -> HashMap<String, f64> {
        self.terms
            .iter()
            .map(|(exponent, coefficient)| (format!("{exponent:?}"), *coefficient))
            .collect::<HashMap<_, _>>()
    }
}

fn parse(key: &str) -> Result<Vec<usize>, Error> {
    let trimmed = key.trim();
    let inner = trimmed
        .strip_prefix('[')
        .and_then(|rest| rest.strip_suffix(']'))
        .ok_or_else(|| Error::Parse {
            key: key.to_string(),
        })?;

    if inner.trim().is_empty() {
        return Ok(Vec::new());
    }

    inner
        .split(',')
        .map(|part| {
            part.trim().parse::<usize>().map_err(|_| Error::Parse {
                key: key.to_string(),
            })
        })
        .collect::<Result<Vec<_>, _>>()
}

impl<'de> serde::Deserialize<'de> for Expression {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let map = HashMap::<String, f64>::deserialize(deserializer)?;
        Self::parse(&map).map_err(serde::de::Error::custom)
    }
}

impl serde::Serialize for Expression {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.representation().serialize(serializer)
    }
}

#[derive(Error, Diagnostic, Debug)]
pub enum Error {
    #[error("failed to parse expression key: {key}")]
    #[diagnostic(
        code(expression::parse),
        help("keys must be JSON arrays of exponents, e.g. \"[2]\" or \"[1,1]\"")
    )]
    Parse { key: String },
}

impl std::fmt::Display for Expression {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{:?}", self.representation())
    }
}
