use super::Trie;
use component::order;
use component::query::Traverse;
use std::collections::VecDeque;

pub struct Depth<'a, T> {
    exploration: Vec<(&'a Trie<T>, Vec<T>, usize)>,
}

impl<'a, T> Depth<'a, T> {
    pub fn new(trie: &'a Trie<T>) -> Self {
        Depth {
            exploration: vec![(trie, Vec::new(), trie.count)],
        }
    }
}

impl<'a, T> Iterator for Depth<'a, T>
where
    T: Clone + Ord,
{
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some((node, path, mut remaining)) = self.exploration.pop() {
            if remaining == 0 {
                for (key, child) in node.children.iter().rev() {
                    let mut child_path = path.clone();
                    child_path.push(key.clone());
                    self.exploration.push((child, child_path, 0));
                }
                remaining = node.count;
            }

            if remaining > 0 {
                if remaining > 1 {
                    self.exploration.push((node, path.clone(), remaining - 1));
                }
                return Some(path);
            }
        }
        None
    }
}

pub struct Breadth<'a, T> {
    exploration: VecDeque<(&'a Trie<T>, Vec<T>, usize)>,
}

impl<'a, T> Breadth<'a, T> {
    pub fn new(trie: &'a Trie<T>) -> Self {
        let mut exploration = VecDeque::new();
        exploration.push_back((trie, Vec::new(), trie.count));
        Breadth { exploration }
    }
}

impl<'a, T> Iterator for Breadth<'a, T>
where
    T: Clone + Ord,
{
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some((node, path, mut remaining)) = self.exploration.pop_front() {
            if remaining == 0 {
                for (key, child) in node.children.iter() {
                    let mut child_path = path.clone();
                    child_path.push(key.clone());
                    self.exploration.push_back((child, child_path, 0));
                }
                remaining = node.count;
            }

            if remaining > 0 {
                if remaining > 1 {
                    self.exploration
                        .push_front((node, path.clone(), remaining - 1));
                }
                return Some(path);
            }
        }
        None
    }
}

impl<T> Traverse<order::Breadth> for Trie<T>
where
    T: Ord + Clone,
{
    type Iterator<'a>
        = Breadth<'a, T>
    where
        Self: 'a;

    fn traverse<'a>(&'a self) -> Self::Iterator<'a> {
        Breadth::new(self)
    }
}

impl<T> Traverse<order::Depth> for Trie<T>
where
    T: Ord + Clone,
{
    type Iterator<'a>
        = Depth<'a, T>
    where
        Self: 'a;

    fn traverse<'a>(&'a self) -> Self::Iterator<'a> {
        Depth::new(self)
    }
}

impl<T> Traverse<()> for Trie<T>
where
    T: Ord + Clone,
{
    type Iterator<'a>
        = Depth<'a, T>
    where
        Self: 'a;

    fn traverse<'a>(&'a self) -> Self::Iterator<'a> {
        Depth::new(self)
    }
}
