use traits::attribute::{Categorized, Contextualized};

use serde::{Deserialize, Serialize};

use std::borrow::Borrow;
use std::collections::VecDeque;
use std::fmt::Debug;
use std::hash::Hash;

#[derive(Debug, Hash, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum Category<T> {
    Attribute(T),
    Context,
    Group,
    Partition,
    #[default]
    Void,
}

pub trait Value: Default + Debug + Clone + Serialize + Eq + Hash {}

impl<Template> Value for Template where Template: Default + Debug + Clone + Serialize + Eq + Hash {}

#[derive(Debug, Default, Eq, Hash, Clone, PartialEq, Serialize, Deserialize)]
pub struct Attribute<Value: crate::Value> {
    category: Category<Value>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    context: Vec<Attribute<Value>>,
}

impl<Value: crate::Value> Categorized for Attribute<Value> {
    type Category = Category<Value>;

    fn category(&self) -> &Self::Category {
        self.category.borrow()
    }
}

impl<Value: crate::Value> Contextualized for Attribute<Value> {
    type Context = Self;
    fn context(&self) -> &[Self::Context] {
        self.context.as_slice()
    }
}

impl<Value: crate::Value> Attribute<Value> {
    pub fn assembler() -> Assembler<Value> {
        Assembler::empty()
    }
}

impl<Value: crate::Value> Attribute<Value> {
    pub fn new(category: Category<Value>, context: Option<Vec<Attribute<Value>>>) -> Self {
        Self {
            category,
            context: context.unwrap_or_default(),
        }
    }
}

pub struct Iterator<'view, Value: crate::Value> {
    queue: VecDeque<&'view Attribute<Value>>,
}

impl<'view, Value: crate::Value> Iterator<'view, Value> {
    pub fn new(source: &'view Attribute<Value>) -> Self {
        Self {
            queue: VecDeque::from(vec![source]),
        }
    }
}

impl<'view, Value: crate::Value> std::iter::Iterator for Iterator<'view, Value> {
    type Item = &'view Attribute<Value>;

    fn next(&mut self) -> Option<Self::Item> {
        self.queue.pop_front().inspect(|attribute| {
            attribute
                .context()
                .iter()
                .for_each(|context| self.queue.push_back(context))
        })
    }
}

impl<Value: crate::Value> Attribute<Value> {
    pub fn breadth(&self) -> Iterator<'_, Value> {
        Iterator::new(self)
    }
}

#[derive(Clone)]
pub struct Assembler<Value: crate::Value> {
    pub category: Option<Category<Value>>,
    pub context: Vec<Attribute<Value>>,
}

impl<Value: crate::Value> Assembler<Value> {
    pub fn new(category: Category<Value>) -> Self {
        Self {
            category: Some(category),
            context: Vec::new(),
        }
    }

    pub fn empty() -> Self {
        Self {
            category: None,
            context: Vec::new(),
        }
    }

    pub fn category(&mut self, category: Category<Value>) -> &mut Self {
        self.category = Some(category);
        self
    }

    pub fn then(&mut self, context: Attribute<Value>) -> &mut Self {
        self.context.push(context);
        self
    }

    pub fn assemble(&self) -> Attribute<Value> {
        Attribute {
            category: self.category.clone().unwrap_or_default(),
            context: self.context.clone(),
        }
    }
}
