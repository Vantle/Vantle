use std::hash::Hash;

use observe::trace;
use record::category::State;
use serde::Serialize;

pub use error;

use component::arena::Valued;

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

impl<T: Eq + Hash + Serialize> Indexed for Valued<T> {
    type Value = T;
    type Identity = usize;
    type Error = error::Missing;

    #[trace(channels = [core])]
    fn value(&self, alias: Self::Identity) -> Result<&Self::Value, Self::Error> {
        let result = self
            .values
            .get(&alias)
            .map(std::sync::Arc::as_ref)
            .ok_or_else(|| error::Missing::element(alias));

        if let Ok(value) = &result {
            record::event!(channels = [arena], id = alias, value = *value);
        }

        result
    }
}

impl<T: Eq + Hash + std::fmt::Debug + Serialize> Aliased for Valued<T> {
    type Value = T;
    type Identity = usize;
    type Error = error::Missing;

    #[trace(channels = [core])]
    fn alias(&self, value: &Self::Value) -> Result<Self::Identity, Self::Error> {
        let result = self
            .indices
            .get(value)
            .copied()
            .ok_or_else(|| error::Missing::element(value));

        if let Ok(id) = &result {
            record::event!(channels = [arena], id = *id, value = value);
        }

        result
    }
}

impl<T: Eq + Hash + Serialize> Allocatable for Valued<T> {
    type Value = T;
    type Identity = usize;
    type Error = error::Error;

    #[trace(channels = [core])]
    fn allocate(&mut self, value: Self::Value) -> Result<Self::Identity, Self::Error> {
        use std::sync::Arc;

        let arc = Arc::new(value);
        if let Some(&id) = self.indices.get(&arc) {
            record::event!(
                channels = [arena],
                id = id,
                value = arc.as_ref(),
                state = State::Existing
            );
            return Ok(id);
        }

        if self.counter == usize::MAX {
            return Err(error::allocation::Allocation::Limit.into());
        }

        let id = self.counter;
        self.counter += 1;
        self.values.insert(id, arc.clone());
        self.indices.insert(arc.clone(), id);

        record::event!(
            channels = [arena],
            id = id,
            value = arc.as_ref(),
            state = State::Created
        );

        Ok(id)
    }
}

impl<T: Eq + Hash + std::fmt::Debug + Serialize> Arena for Valued<T> {}
