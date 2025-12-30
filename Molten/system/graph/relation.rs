use std::collections::BTreeSet;

use observe::trace;
use serde::{Serialize, de::DeserializeOwned};

use component::graph::relation::{Constructor, Related};

pub trait Linked {
    type Element;
    type Stream: IntoIterator<Item = Self::Element>;

    fn descendants(&self, from: &Self::Element) -> Option<Self::Stream>;
    fn predecessors(&self, from: &Self::Element) -> Option<Self::Stream>;
}

impl<Label: Eq + Ord + Default + Clone + Serialize + DeserializeOwned> Linked for Related<Label> {
    type Element = Label;
    type Stream = BTreeSet<Label>;

    #[trace(channels = [core])]
    fn descendants(&self, from: &Self::Element) -> Option<Self::Stream> {
        self.adjacency.get(from).cloned()
    }

    #[trace(channels = [core])]
    fn predecessors(&self, from: &Self::Element) -> Option<Self::Stream> {
        let predecessors = self
            .adjacency
            .iter()
            .filter(|(_, descendants)| descendants.contains(from))
            .map(|(element, _)| element.clone())
            .collect::<BTreeSet<_>>();

        (!predecessors.is_empty()).then_some(predecessors)
    }
}

pub trait Relate<Label> {
    fn relate(&mut self, label: &Label, relation: &Label) -> &mut Self
    where
        Label: Clone;
}

impl<Label: Eq + Ord + Serialize + DeserializeOwned + Clone> Relate<Label> for Related<Label> {
    #[trace(channels = [core])]
    fn relate(&mut self, label: &Label, relation: &Label) -> &mut Self
    where
        Label: Clone,
    {
        self.adjacency
            .entry(label.clone())
            .or_default()
            .insert(relation.clone());
        self
    }
}

impl<Label: Default + Eq + Ord + Clone> Relate<Label> for Constructor<Label> {
    #[trace(channels = [core])]
    fn relate(&mut self, label: &Label, relation: &Label) -> &mut Self
    where
        Label: Clone,
    {
        self.adjacency
            .entry(label.clone())
            .or_default()
            .insert(relation.clone());
        self
    }
}
