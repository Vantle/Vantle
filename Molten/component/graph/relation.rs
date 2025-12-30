use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize, de::DeserializeOwned};
use serde_with::serde_as;

#[serde_as]
#[derive(Debug, Default, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Related<Label: Eq + Ord + Serialize + DeserializeOwned> {
    #[serde_as(as = "Vec<(_, _)>")]
    pub adjacency: BTreeMap<Label, BTreeSet<Label>>,
}

impl<Label: Eq + Ord + Serialize + DeserializeOwned> Related<Label> {
    #[must_use]
    pub fn new(adjacency: BTreeMap<Label, BTreeSet<Label>>) -> Self {
        Related { adjacency }
    }

    pub fn iter(&self) -> std::collections::btree_map::Iter<'_, Label, BTreeSet<Label>> {
        self.adjacency.iter()
    }
}

#[derive(Debug, Default, Clone)]
pub struct Constructor<Label: Default + Eq + Ord> {
    pub adjacency: BTreeMap<Label, BTreeSet<Label>>,
}

impl<Label: Default + Eq + Ord> Constructor<Label> {
    #[must_use]
    pub fn new() -> Self {
        Constructor {
            adjacency: BTreeMap::new(),
        }
    }
}

impl<Label: Default + Eq + Ord + Clone + Serialize + DeserializeOwned> From<Constructor<Label>>
    for Related<Label>
{
    fn from(constructor: Constructor<Label>) -> Self {
        Related {
            adjacency: constructor.adjacency,
        }
    }
}

impl<Label: Default + Eq + Ord + Clone + Serialize + DeserializeOwned> From<&Constructor<Label>>
    for Related<Label>
{
    fn from(constructor: &Constructor<Label>) -> Self {
        Related {
            adjacency: constructor.adjacency.clone(),
        }
    }
}

impl<'a, Label: Eq + Ord + Serialize + DeserializeOwned> IntoIterator for &'a Related<Label> {
    type Item = (&'a Label, &'a BTreeSet<Label>);
    type IntoIter = std::collections::btree_map::Iter<'a, Label, BTreeSet<Label>>;

    fn into_iter(self) -> Self::IntoIter {
        self.adjacency.iter()
    }
}

impl<Label: Eq + Ord + Serialize + DeserializeOwned> IntoIterator for Related<Label> {
    type Item = (Label, BTreeSet<Label>);
    type IntoIter = std::collections::btree_map::IntoIter<Label, BTreeSet<Label>>;

    fn into_iter(self) -> Self::IntoIter {
        self.adjacency.into_iter()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(bound(serialize = "T: Serialize", deserialize = "T: DeserializeOwned"))]
pub struct Edge<T: Clone + Eq + Ord + Serialize + DeserializeOwned> {
    pub source: T,
    pub sink: T,
}
