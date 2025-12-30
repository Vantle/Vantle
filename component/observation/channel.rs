use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Channel {
    pub name: String,
    pub weight: u8,
}

#[derive(thiserror::Error, miette::Diagnostic, Debug, Clone, PartialEq, Eq)]
pub enum Error {
    #[error("duplicate channel: {channel}")]
    #[diagnostic(
        code(observation::channel::duplicate),
        help("remove duplicate channel definition")
    )]
    Duplicate { channel: String },
}

impl Channel {
    pub fn parse(input: &str) -> Result<Vec<Self>, Error> {
        let mut map = BTreeMap::<String, u8>::new();

        for segment in input.split(',') {
            let trimmed = segment.trim();
            if trimmed.is_empty() {
                continue;
            }

            let (name, weight) = match trimmed.split_once(':') {
                Some((n, w)) => (n.trim(), w.trim().parse().unwrap_or(1)),
                None => (trimmed, 1),
            };

            if map.contains_key(name) {
                return Err(Error::Duplicate {
                    channel: name.to_string(),
                });
            }
            map.insert(name.to_string(), weight);
        }

        Ok(map
            .into_iter()
            .map(|(name, weight)| Channel { name, weight })
            .collect())
    }

    #[must_use]
    pub fn serialize(channels: &[Self]) -> String {
        channels
            .iter()
            .map(|c| format!("{}:{}", c.name, c.weight))
            .collect::<Vec<_>>()
            .join(",")
    }

    #[must_use]
    pub fn matches(channels: &[Self], filter: &[&str]) -> bool {
        channels.iter().any(|c| filter.contains(&c.name.as_str()))
    }
}
