use observe::trace;

use component::graph::attribute::{Attribute, Value};
use component::graph::index::Index as Data;
use component::graph::state::wave::Wave;

pub trait Index {
    type Value;
    fn new(value: Attribute<Self::Value>) -> Result<Self, arena::error::Error>
    where
        Self: Sized;
    fn allocate(
        &mut self,
        value: Attribute<Self::Value>,
    ) -> Result<(usize, Wave<usize>), arena::error::Error>;
}

impl<T> Index for Data<T>
where
    T: Value,
{
    type Value = T;

    #[trace(channels = [core])]
    fn new(value: Attribute<Self::Value>) -> Result<Self, arena::error::Error> {
        let mut index = Self::default();
        index.allocate(value)?;
        Ok(index)
    }

    #[trace(channels = [core])]
    fn allocate(
        &mut self,
        value: Attribute<Self::Value>,
    ) -> Result<(usize, Wave<usize>), arena::error::Error> {
        let id = attribute::Allocatable::allocate(&mut self.arena, value.clone())?;
        mutator::relate(&value, &self.arena, &mut self.relations)?;
        let signal = constructor::signal(&value, &self.arena)?;
        Ok((id, signal))
    }
}
