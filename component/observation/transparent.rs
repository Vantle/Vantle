pub use category;
pub use collector;
pub use serialize;
pub use span;

#[macro_export]
macro_rules! event {
    (channels = [$($channel:ident $(: $weight:literal)?),* $(,)?], $($key:ident = $value:expr),* $(,)?) => {{
        let channels = $crate::channels!($($channel $(: $weight)?),*);
        $crate::collector::tracing::info!(channels = %channels, $($key = %$crate::serialize::json(&$value)),*)
    }};
}

#[macro_export]
macro_rules! channels {
    ($($channel:ident $(: $weight:literal)?),* $(,)?) => {{
        let mut parts = Vec::<String>::new();
        $(
            let weight: u8 = $crate::channels!(@weight $($weight)?);
            parts.push(format!("{}:{}", stringify!($channel), weight));
        )*
        parts.join(",")
    }};
    (@weight $weight:literal) => { $weight };
    (@weight) => { 1u8 };
}

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
