use std::collections::{HashMap, HashSet};
use std::hash::Hash;

use serde::{Deserialize, Serialize};
use serde_with::serde_as;

#[serde_as]
#[derive(Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct Related<Label: Eq + Hash + Serialize + for<'a> Deserialize<'a>> {
    #[serde_as(as = "Vec<(_, _)>")]
    adjacency: HashMap<Label, HashSet<Label>>,
}

impl<Label: Eq + Hash + Serialize + for<'a> Deserialize<'a>> Related<Label> {
    pub fn new(adjacency: HashMap<Label, HashSet<Label>>) -> Self {
        Related { adjacency }
    }

    pub fn iter(&self) -> impl Iterator<Item = (&Label, &HashSet<Label>)> {
        self.adjacency.iter()
    }
}

#[derive(Default)]
pub struct Constructor<Label: Default + Eq + Hash> {
    adjacency: HashMap<Label, HashSet<Label>>,
}

impl<Label: Default + Eq + Hash + Clone + Serialize + for<'a> Deserialize<'a>> Constructor<Label> {
    pub fn relate(&mut self, label: &Label, relation: &Label) -> &mut Self {
        self.adjacency
            .entry(label.clone())
            .or_default()
            .insert(relation.clone());
        self
    }

    pub fn identified(&self) -> Related<Label> {
        Related {
            adjacency: self.adjacency.clone(),
        }
    }
}

impl<Label: Eq + Hash + Default + Clone + Serialize + for<'a> Deserialize<'a>> node::Aliased
    for Related<Label>
{
    type Value = HashSet<Label>;
    type Identity = Label;
    type Error = ();

    fn value(&self, alias: Self::Identity) -> Result<&Self::Value, Self::Error> {
        self.adjacency.get(&alias).ok_or(())
    }

    fn alias(&self, _value: &Self::Value) -> Result<Self::Identity, Self::Error> {
        // This is a simplified implementation - in practice you'd need to find the key for the value
        todo!("Implement reverse lookup from value to alias")
    }
}

impl<Label: Eq + Hash + Default + Clone + Serialize + for<'a> Deserialize<'a>> link::Linked
    for Related<Label>
{
    type Element = Label;
    type Stream = HashSet<Label>;

    fn descendents(&self, from: &Self::Element) -> Option<Self::Stream> {
        self.adjacency.get(from).cloned()
    }

    fn predecessors(&self, from: &Self::Element) -> Option<Self::Stream> {
        // Find all elements that have 'from' as a descendent
        let predecessors: HashSet<Label> = self
            .adjacency
            .iter()
            .filter(|(_, descendents)| descendents.contains(from))
            .map(|(element, _)| element.clone())
            .collect();

        if predecessors.is_empty() {
            None
        } else {
            Some(predecessors)
        }
    }
}
