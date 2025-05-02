use attributes::{Categorized, Contextualized, Valued};

use serde::{Deserialize, Serialize};

use std::borrow::Borrow;
use std::collections::VecDeque;
use std::fmt::Debug;
use std::hash::Hash;

#[derive(Debug, Default, Hash, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum Category {
    Attribute,
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
    category: Category,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    value: Option<Value>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    context: Vec<Attribute<Value>>,
}

impl<Value: crate::Value> Categorized for Attribute<Value> {
    type Category = Category;

    fn category(&self) -> &Self::Category {
        self.category.borrow()
    }
}

impl<Value: crate::Value> Valued for Attribute<Value> {
    type Value = Value;

    fn value(&self) -> &Self::Value {
        self.value.as_ref().unwrap().borrow()
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
    pub fn new(
        category: Category,
        value: Option<Value>,
        context: Option<Vec<Attribute<Value>>>,
    ) -> Self {
        Self {
            category,
            value,
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
    pub category: Option<Category>,
    pub value: Option<Value>,
    pub context: Vec<Attribute<Value>>,
}

impl<Value: crate::Value> Assembler<Value> {
    pub fn new(category: Category) -> Self {
        Self {
            category: Some(category),
            value: None,
            context: Vec::new(),
        }
    }

    pub fn empty() -> Self {
        Self {
            category: None,
            value: None,
            context: Vec::new(),
        }
    }

    pub fn category(&mut self, category: Category) -> &mut Self {
        self.category = Some(category);
        self
    }

    pub fn value(&mut self, value: Value) -> &mut Self {
        self.value = Some(value);
        self
    }

    pub fn then(&mut self, context: Attribute<Value>) -> &mut Self {
        self.context.push(context);
        self
    }

    pub fn assemble(&self) -> Attribute<Value> {
        Attribute {
            category: self.category.unwrap_or(Category::Context),
            value: self.value.clone(),
            context: self.context.clone(),
        }
    }
}
