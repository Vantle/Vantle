//! Case handling module that re-exports types from focused libraries.

// Re-export commonly used items at the module level for convenience
pub use error::{Error, Result};
pub use schema::{Case, Cases, Function};
pub use types::{Callable, Counters, Functions, Structs};
