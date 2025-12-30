pub use channel;

use channel::Channel;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

pub type Predicate = Arc<dyn Fn(&[Channel]) -> bool + Send + Sync>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Identifier {
    pub trace: u64,
    pub span: u64,
    pub parent: Option<u64>,
}

impl Identifier {
    #[inline]
    #[must_use]
    pub fn root(trace: u64, span: u64) -> Self {
        Self {
            trace,
            span,
            parent: None,
        }
    }

    #[inline]
    #[must_use]
    pub fn child(trace: u64, span: u64, parent: u64) -> Self {
        Self {
            trace,
            span,
            parent: Some(parent),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Level {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

impl Default for Level {
    fn default() -> Self {
        Self::Info
    }
}

impl From<tracing::Level> for Level {
    fn from(level: tracing::Level) -> Self {
        match level {
            tracing::Level::TRACE => Self::Trace,
            tracing::Level::DEBUG => Self::Debug,
            tracing::Level::INFO => Self::Info,
            tracing::Level::WARN => Self::Warn,
            tracing::Level::ERROR => Self::Error,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Metadata {
    pub target: String,
    pub name: String,
    pub level: Level,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Value {
    Signed(i64),
    Unsigned(u64),
    Boolean(bool),
    Text(String),
    Serialized(Vec<u8>),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Field {
    pub name: String,
    pub value: Value,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Begin {
    pub timestamp: u64,
    pub fields: Vec<Field>,
}

impl Begin {
    #[inline]
    #[must_use]
    pub fn now(fields: Vec<Field>) -> Self {
        Self {
            timestamp: timestamp(),
            fields,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct End {
    pub timestamp: u64,
}

impl End {
    #[inline]
    #[must_use]
    pub fn now() -> Self {
        Self {
            timestamp: timestamp(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Span {
    pub id: Identifier,
    pub metadata: Metadata,
    pub channels: Vec<Channel>,
    pub lifecycle: Lifecycle,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Lifecycle {
    Begin(Begin),
    End(End),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Event {
    pub parent: Option<u64>,
    pub metadata: Metadata,
    pub channels: Vec<Channel>,
    pub timestamp: u64,
    pub fields: Vec<Field>,
}

impl Event {
    #[inline]
    #[must_use]
    pub fn now(
        parent: Option<u64>,
        metadata: Metadata,
        channels: Vec<Channel>,
        fields: Vec<Field>,
    ) -> Self {
        Self {
            parent,
            metadata,
            channels,
            timestamp: timestamp(),
            fields,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Snapshot {
    pub timestamp: u64,
    pub state: Vec<u8>,
    pub trigger: String,
}

impl Snapshot {
    #[inline]
    #[must_use]
    pub fn now(state: Vec<u8>, trigger: String) -> Self {
        Self {
            timestamp: timestamp(),
            state,
            trigger,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Update {
    Span(Span),
    Event(Event),
    Snapshot(Snapshot),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Command {
    Pause,
    Resume,
    Seek(u64),
    Filter { targets: Vec<String>, level: Level },
    Snapshot,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Record {
    pub path: String,
    pub count: u64,
    pub duration: u64,
}

#[inline]
#[must_use]
pub fn timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_micros().try_into().unwrap_or(u64::MAX))
        .unwrap_or(0)
}

#[inline]
#[must_use]
pub fn generate() -> u64 {
    use std::hash::{Hash, Hasher};
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    timestamp().hash(&mut hasher);
    std::thread::current().id().hash(&mut hasher);
    hasher.finish()
}

#[must_use]
pub fn coerce(name: &str, text: String) -> Field {
    let parsed = serde_json::from_str::<serde_json::Value>(&text);

    let value = match parsed {
        Ok(serde_json::Value::Number(n)) => {
            if let Some(i) = n.as_i64() {
                Value::Signed(i)
            } else if let Some(u) = n.as_u64() {
                Value::Unsigned(u)
            } else {
                Value::Text(text)
            }
        }
        Ok(serde_json::Value::Bool(b)) => Value::Boolean(b),
        Ok(serde_json::Value::String(s)) => Value::Text(s),
        Ok(serde_json::Value::Null) => Value::Text("null".to_string()),
        Ok(serde_json::Value::Array(_) | serde_json::Value::Object(_)) => {
            Value::Serialized(text.into_bytes())
        }
        Err(_) => Value::Text(text),
    };

    Field {
        name: name.to_string(),
        value,
    }
}
