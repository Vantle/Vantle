use std::hash::Hash;

use observe::trace;

pub use component::arena::Valued;
pub use error;

pub trait Indexed {
    type Value;
    type Identity;
    type Error;
    fn value(&self, alias: Self::Identity) -> Result<&Self::Value, Self::Error>;
}

pub trait Aliased {
    type Value;
    type Identity;
    type Error;
    fn alias(&self, value: &Self::Value) -> Result<Self::Identity, Self::Error>;
}

pub trait Allocatable {
    type Value;
    type Identity;
    type Error;
    fn allocate(&mut self, value: Self::Value) -> Result<Self::Identity, Self::Error>;
}

pub trait Arena: Allocatable + Aliased + Indexed {}

impl<T: Eq + Hash> Indexed for Valued<T> {
    type Value = T;
    type Identity = usize;
    type Error = error::Missing;

    #[trace(channels = [core])]
    fn value(&self, alias: Self::Identity) -> Result<&Self::Value, Self::Error> {
        self.values
            .get(&alias)
            .map(std::sync::Arc::as_ref)
            .ok_or_else(|| error::Missing::element(alias))
    }
}

impl<T: Eq + Hash + std::fmt::Debug> Aliased for Valued<T> {
    type Value = T;
    type Identity = usize;
    type Error = error::Missing;

    #[trace(channels = [core])]
    fn alias(&self, value: &Self::Value) -> Result<Self::Identity, Self::Error> {
        self.indices
            .iter()
            .find(|(k, _)| k.as_ref() == value)
            .map(|(_, &v)| v)
            .ok_or_else(|| error::Missing::element(value))
    }
}

impl<T: Eq + Hash> Allocatable for Valued<T> {
    type Value = T;
    type Identity = usize;
    type Error = error::Error;

    #[trace(channels = [core])]
    fn allocate(&mut self, value: Self::Value) -> Result<Self::Identity, Self::Error> {
        use std::sync::Arc;

        let arc = Arc::new(value);
        if let Some(&id) = self.indices.get(&arc) {
            return Ok(id);
        }

        if self.counter == usize::MAX {
            return Err(error::allocation::Allocation::Limit.into());
        }

        let id = self.counter;
        self.counter += 1;
        self.values.insert(id, arc.clone());
        self.indices.insert(arc, id);
        Ok(id)
    }
}

impl<T: Eq + Hash + std::fmt::Debug> Arena for Valued<T> {}
