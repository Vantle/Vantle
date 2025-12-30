use std::fmt::Debug;
use std::hash::Hash;

use assemble::Assemble;
use serde::{Deserialize, Serialize, de::DeserializeOwned};

pub trait Value: Default + Debug + Clone + Serialize + DeserializeOwned + Eq + Hash {}

impl<Template> Value for Template where
    Template: Default + Debug + Clone + Serialize + DeserializeOwned + Eq + Hash
{
}

#[derive(Debug, Hash, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(bound = "T: Value")]
pub enum Category<T> {
    Attribute(T),
    Context,
    Group,
    Partition,
    #[default]
    Void,
}

#[derive(Debug, Default, Eq, Hash, Clone, PartialEq, Serialize, Deserialize)]
#[serde(bound = "Value: self::Value")]
pub struct Attribute<Value> {
    pub category: Category<Value>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub context: Vec<Attribute<Value>>,
}

#[derive(Debug, Default, Clone)]
pub struct Assembler<Value> {
    pub category: Option<Category<Value>>,
    pub context: Vec<Attribute<Value>>,
}

impl<Value: self::Value> Assembler<Value> {
    #[must_use]
    pub fn new(category: Category<Value>) -> Self {
        Self {
            category: Some(category),
            context: Vec::new(),
        }
    }

    #[must_use]
    pub fn empty() -> Self {
        Self {
            category: None,
            context: Vec::new(),
        }
    }

    pub fn context(&mut self, attribute: Attribute<Value>) {
        self.context.push(attribute);
    }

    #[must_use]
    pub fn then(&mut self, attribute: Attribute<Value>) -> &mut Self {
        self.context.push(attribute);
        self
    }

    #[must_use]
    pub fn category(mut self, category: Category<Value>) -> Self {
        self.category = Some(category);
        self
    }
}

impl<Value: self::Value> Assemble for Assembler<Value> {
    type Output = Attribute<Value>;

    fn assemble(self) -> Self::Output {
        Attribute {
            category: self.category.unwrap_or_default(),
            context: self.context,
        }
    }
}
