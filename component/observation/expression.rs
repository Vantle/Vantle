use std::sync::Arc;

use channel::Channel;
use stream::Predicate;

use tag::Expression;

pub trait Filterable {
    fn channels(&self, channels: &[Channel]) -> bool;
    fn predicate(self) -> Predicate;
}

impl Filterable for Expression {
    fn channels(&self, channels: &[Channel]) -> bool {
        let names = channels.iter().map(|c| &c.name).collect::<Vec<_>>();
        self.evaluate(&names)
    }

    fn predicate(self) -> Predicate {
        Arc::new(move |channels| self.channels(channels))
    }
}
