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

impl<Label: Eq + Hash + Default + Clone + Serialize + for<'a> Deserialize<'a>> relations::Identified
    for Related<Label>
{
    type Identity = Label;
    type Stream = HashSet<Label>;
    fn transitions(&self, label: &Self::Identity) -> Self::Stream {
        self.adjacency
            .get(label)
            .unwrap_or(&Self::Stream::default())
            .clone()
    }
}
