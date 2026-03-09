use std::marker::PhantomData;

pub struct Guard(PhantomData<()>);

pub use category;
pub use serialize;

#[inline(always)]
pub fn emit<T>(_: &T) {}

#[macro_export]
macro_rules! enter {
    ($name:expr) => {{
        let _ = $name;
        $crate::Guard(std::marker::PhantomData)
    }};
}

#[macro_export]
macro_rules! channels {
    ($($channel:ident $(: $weight:literal)?),* $(,)?) => {
        ""
    };
}

#[macro_export]
macro_rules! event {
    (channels = [$($channel:ident $(: $weight:literal)?),* $(,)?], $($field:tt)*) => {{
        $(let _ = stringify!($channel);)*
        $crate::event!($($field)*)
    }};
    ($($key:ident = $value:expr),* $(,)?) => {{ $(let _ = &$value;)* }};
    ($($arg:tt)*) => {{ let _ = format_args!($($arg)*); }};
}

#[macro_export]
macro_rules! trace {
    ($($key:ident = $value:expr),* $(,)?) => {{ $(let _ = &$value;)* }};
    ($($arg:tt)*) => {{ let _ = format_args!($($arg)*); }};
}

#[macro_export]
macro_rules! debug {
    ($($key:ident = $value:expr),* $(,)?) => {{ $(let _ = &$value;)* }};
    ($($arg:tt)*) => {{ let _ = format_args!($($arg)*); }};
}

#[macro_export]
macro_rules! info {
    ($($key:ident = $value:expr),* $(,)?) => {{ $(let _ = &$value;)* }};
    ($($arg:tt)*) => {{ let _ = format_args!($($arg)*); }};
}

#[macro_export]
macro_rules! warn {
    ($($key:ident = $value:expr),* $(,)?) => {{ $(let _ = &$value;)* }};
    ($($arg:tt)*) => {{ let _ = format_args!($($arg)*); }};
}

#[macro_export]
macro_rules! error {
    ($($key:ident = $value:expr),* $(,)?) => {{ $(let _ = &$value;)* }};
    ($($arg:tt)*) => {{ let _ = format_args!($($arg)*); }};
}
