pub use context;
pub use group;

pub const PARTITION: u8 = b',';
pub const CONTINUATION: u8 = b'.';

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Syntax {
    Contexting,
    Contexted,
    Grouping,
    Grouped,
    Partition,
    Continuation,
    Element(u8),
}

impl From<u8> for Syntax {
    fn from(value: u8) -> Self {
        match value {
            value if value == context::INITIAL => Syntax::Contexting,
            value if value == context::TERMINAL => Syntax::Contexted,
            value if value == group::INITIAL => Syntax::Grouping,
            value if value == group::TERMINAL => Syntax::Grouped,
            value if value == PARTITION => Syntax::Partition,
            value if value == CONTINUATION => Syntax::Continuation,
            other => Syntax::Element(other),
        }
    }
}
