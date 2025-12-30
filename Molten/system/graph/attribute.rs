use std::collections::VecDeque;
use std::sync::Arc;

use component::graph::attribute::{Attribute as Data, Category, Value};
use observe::trace;
use record::trace as log;
use valued::Valued;

pub trait Categorized {
    type Category;
    fn category(&self) -> &Self::Category;
}

pub trait Contextualized
where
    Self: Sized,
{
    type Context;
    fn context(&self) -> &[Self::Context];

    fn breadth(&self) -> Breadth<'_, Self> {
        Breadth {
            queue: VecDeque::from(vec![self]),
        }
    }

    fn depth(&self) -> Depth<'_, Self> {
        Depth { stack: vec![self] }
    }
}

pub trait Attribute: Categorized + Contextualized {
    type Value: self::Value;

    fn root(&self) -> &Data<Self::Value>;
}

pub trait Arena
where
    Self: Contextualized<Context = Self> + Clone + Eq + std::hash::Hash + std::fmt::Debug,
{
    fn arena(&self) -> Result<Valued<Self>, arena::error::Error>;
}

pub trait Allocatable {
    type Value;
    type Error;
    fn allocate(&mut self, value: Self::Value) -> Result<usize, Self::Error>;
}

pub struct Breadth<'view, T: Contextualized> {
    queue: VecDeque<&'view T>,
}

pub struct Depth<'view, T: Contextualized> {
    stack: Vec<&'view T>,
}

impl<Value: self::Value> Categorized for Data<Value> {
    type Category = Category<Value>;

    #[trace(channels = [core])]
    fn category(&self) -> &Self::Category {
        &self.category
    }
}

impl<Value: self::Value> Contextualized for Data<Value> {
    type Context = Data<Value>;

    #[trace(channels = [core])]
    fn context(&self) -> &[Self::Context] {
        self.context.as_slice()
    }
}

impl<Value: self::Value> Attribute for Data<Value> {
    type Value = Value;

    #[trace(channels = [core])]
    fn root(&self) -> &Data<Self::Value> {
        self
    }
}

impl<T> Arena for T
where
    T: Contextualized<Context = T> + Clone + Eq + std::hash::Hash + std::fmt::Debug,
{
    #[trace(channels = [core])]
    fn arena(&self) -> Result<Valued<Self>, arena::error::Error> {
        let mut valued = Valued::default();
        Allocatable::allocate(&mut valued, self.clone())?;
        Ok(valued)
    }
}

impl<'view, T: Contextualized<Context = T>> Iterator for Breadth<'view, T> {
    type Item = &'view T;

    #[trace(channels = [core])]
    fn next(&mut self) -> Option<Self::Item> {
        self.queue.pop_front().inspect(|item| {
            item.context()
                .iter()
                .for_each(|context| self.queue.push_back(context));
        })
    }
}

impl<'view, T: Contextualized<Context = T>> Iterator for Depth<'view, T> {
    type Item = &'view T;

    #[trace(channels = [core])]
    fn next(&mut self) -> Option<Self::Item> {
        self.stack.pop().inspect(|item| {
            item.context()
                .iter()
                .rev()
                .for_each(|context| self.stack.push(context));
        })
    }
}

impl<T> Allocatable for Valued<T>
where
    T: Contextualized<Context = T> + Clone + Eq + std::hash::Hash + std::fmt::Debug,
{
    type Value = T;
    type Error = arena::error::Error;

    #[trace(channels = [core])]
    fn allocate(&mut self, value: Self::Value) -> Result<usize, Self::Error> {
        log!("Allocating attribute {:?}", value);

        let reference = Arc::new(value.clone());

        if let Some(&existing) = self.indices.get(&reference) {
            log!(
                "Unified constraint {:?} on allocation {:?}.",
                value,
                existing
            );
            return Ok(existing);
        }

        if self.counter == usize::MAX {
            return Err(arena::error::allocation::Allocation::Limit.into());
        }

        let root = self.counter;
        self.counter += 1;

        self.indices.insert(reference.clone(), root);
        self.values.insert(root, reference);

        for item in value.breadth() {
            let reference = Arc::new(item.clone());

            if self.indices.contains_key(&reference) {
                continue;
            }

            if self.counter == usize::MAX {
                return Err(arena::error::allocation::Allocation::Limit.into());
            }

            let index = self.counter;
            self.counter += 1;

            self.indices.insert(reference.clone(), index);
            self.values.insert(index, reference);
        }

        Ok(root)
    }
}
