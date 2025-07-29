//! Syntax definitions for the symbolic graph constructor component.

/// Syntax character mappings for symbolic graph construction
pub mod syntax {
    /// Context syntax characters
    pub mod context {
        /// Opening context character: '['
        pub const INITIAL: u8 = b'[';
        /// Closing context character: ']'
        pub const TERMINAL: u8 = b']';
    }

    /// Group syntax characters
    pub mod group {
        /// Opening group character: '('
        pub const INITIAL: u8 = b'(';
        /// Closing group character: ')'
        pub const TERMINAL: u8 = b')';
    }

    /// Partition separator character: ','
    pub const PARTITION: u8 = b',';

    /// Continuation character: '.'
    pub const CONTINUATION: u8 = b'.';
}

/// Enum representing all syntax characters for pattern matching
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
