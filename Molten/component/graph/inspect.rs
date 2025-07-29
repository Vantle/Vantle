use std::fmt;
use std::hash::Hash;

use serde::{Deserialize, Serialize};

use arena::Valued;
use matrix::Related;
use traits::node::{Aliased, Valued as _};

pub struct View<'a, A, L>
where
    A: fmt::Debug + Eq + Hash,
    L: Eq + Hash + Serialize + for<'de> Deserialize<'de>,
{
    arena: &'a Valued<A>,
    graph: &'a Related<L>,
}

impl<'a, A, L> View<'a, A, L>
where
    A: fmt::Debug + Eq + Hash,
    L: Eq + Hash + Clone + Serialize + for<'de> Deserialize<'de>,
{
    pub fn new(graph: &'a Related<L>, arena: &'a Valued<A>) -> Self {
        Self { arena, graph }
    }

    pub fn label(&self, id: usize) -> Option<&A> {
        self.arena.value(id).ok()
    }

    pub fn id(&self, label: &A) -> Option<usize> {
        self.arena.alias(label).ok()
    }
}

impl<'a, A, L> fmt::Display for View<'a, A, L>
where
    A: fmt::Debug + Eq + Hash,
    L: fmt::Debug + Eq + Hash + Serialize + for<'de> Deserialize<'de>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (label, targets) in self.graph {
            writeln!(f, "{:#?} -> {:#?}", label, targets)?;
        }
        Ok(())
    }
}
