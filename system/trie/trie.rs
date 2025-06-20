use component::mutate::{Delete, Insert};
use component::query::Search;
use std::borrow::Borrow;
use std::collections::BTreeMap;

pub mod traverse;
pub use traverse::{Breadth, Depth};

#[derive(Clone)]
pub struct Trie<T> {
    children: BTreeMap<T, Trie<T>>,
    count: usize,
}

impl<T> Trie<T>
where
    T: Ord,
{
    pub fn new<I>(items: I) -> Self
    where
        T: Clone,
        I: IntoIterator<Item: IntoIterator<Item: Borrow<T>>>,
    {
        items.into_iter().fold(Self::default(), |mut trie, item| {
            trie.insert(item);
            trie
        })
    }

    pub fn quantity(&self) -> usize {
        self.count
    }
}

impl<T> Default for Trie<T>
where
    T: Ord,
{
    fn default() -> Self {
        Trie {
            children: BTreeMap::new(),
            count: 0,
        }
    }
}

impl<T, I> Insert<I> for Trie<T>
where
    T: Ord + Clone,
    I: IntoIterator<Item: Borrow<T>>,
{
    type Return<'a>
        = &'a Self
    where
        Self: 'a;

    fn insert<'a>(&'a mut self, value: I) -> Self::Return<'a> {
        let mut path = value.into_iter();
        match path.next() {
            None => {
                self.count += 1;
                self
            }
            Some(first) => self
                .children
                .entry(first.borrow().clone())
                .or_default()
                .insert(path),
        }
    }
}

impl<T, I> Delete<I> for Trie<T>
where
    T: Ord,
    I: IntoIterator<Item: Borrow<T>>,
{
    type Return<'a>
        = Option<&'a Self>
    where
        Self: 'a;

    fn delete<'a>(&'a mut self, value: I) -> Self::Return<'a> {
        let mut path = value.into_iter();
        match path.next() {
            None => {
                if self.count > 0 {
                    self.count -= 1;
                    Some(self)
                } else {
                    None
                }
            }
            Some(first) => {
                let key = first.borrow();
                let child = self.children.get_mut(key)?;
                if child.delete(path).is_some() {
                    if child.count == 0 && child.children.is_empty() {
                        self.children.remove(key);
                    }
                    Some(self)
                } else {
                    None
                }
            }
        }
    }
}

impl<T, I> Search<I> for Trie<T>
where
    T: Ord,
    I: IntoIterator<Item: Borrow<T>>,
{
    type Return<'a>
        = Option<&'a Self>
    where
        Self: 'a;

    fn search<'a>(&'a self, query: I) -> Self::Return<'a> {
        let mut path = query.into_iter();
        match path.next() {
            None => Some(self),
            Some(first) => self.children.get(first.borrow())?.search(path),
        }
    }
}
