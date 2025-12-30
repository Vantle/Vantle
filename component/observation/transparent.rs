pub use collector;
pub use span;

#[macro_export]
macro_rules! trace {
    ($($arg:tt)*) => {{
        $crate::collector::tracing::trace!($($arg)*)
    }};
}

#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {{
        $crate::collector::tracing::debug!($($arg)*)
    }};
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {{
        $crate::collector::tracing::info!($($arg)*)
    }};
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {{
        $crate::collector::tracing::warn!($($arg)*)
    }};
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {{
        $crate::collector::tracing::error!($($arg)*)
    }};
}
