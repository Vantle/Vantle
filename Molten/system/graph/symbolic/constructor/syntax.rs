pub mod syntax {
    pub mod context {
        pub const INITIAL: u8 = b'[';
        pub const TERMINAL: u8 = b']';
    }

    pub mod group {
        pub const INITIAL: u8 = b'(';
        pub const TERMINAL: u8 = b')';
    }

    pub const PARTITION: u8 = b',';

    pub const CONTINUATION: u8 = b'.';
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Syntax {
    ContextInitial,
    ContextTerminal,
    GroupInitial,
    GroupTerminal,
    Partition,
    Continuation,
    Other(u8),
}

impl From<u8> for Syntax {
    fn from(value: u8) -> Self {
        match value {
            value if value == syntax::context::INITIAL => Syntax::ContextInitial,
            value if value == syntax::context::TERMINAL => Syntax::ContextTerminal,
            value if value == syntax::group::INITIAL => Syntax::GroupInitial,
            value if value == syntax::group::TERMINAL => Syntax::GroupTerminal,
            value if value == syntax::PARTITION => Syntax::Partition,
            value if value == syntax::CONTINUATION => Syntax::Continuation,
            other => Syntax::Other(other),
        }
    }
}
