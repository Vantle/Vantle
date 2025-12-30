use std::marker::PhantomData;

pub struct Guard(PhantomData<()>);

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
macro_rules! event {
    ($($key:ident = $value:expr),* $(,)?) => {{ $(let _ = &$value;)* }};
    ($fmt:literal $(, $args:expr)* $(,)?) => {{ $(let _ = &$args;)* }};
    ($($arg:tt)*) => {{}};
}

#[macro_export]
macro_rules! trace {
    ($($key:ident = $value:expr),* $(,)?) => {{ $(let _ = &$value;)* }};
    ($fmt:literal $(, $args:expr)* $(,)?) => {{ $(let _ = &$args;)* }};
    ($($arg:tt)*) => {{}};
}

#[macro_export]
macro_rules! debug {
    ($($key:ident = $value:expr),* $(,)?) => {{ $(let _ = &$value;)* }};
    ($fmt:literal $(, $args:expr)* $(,)?) => {{ $(let _ = &$args;)* }};
    ($($arg:tt)*) => {{}};
}

#[macro_export]
macro_rules! info {
    ($($key:ident = $value:expr),* $(,)?) => {{ $(let _ = &$value;)* }};
    ($fmt:literal $(, $args:expr)* $(,)?) => {{ $(let _ = &$args;)* }};
    ($($arg:tt)*) => {{}};
}

#[macro_export]
macro_rules! warn {
    ($($key:ident = $value:expr),* $(,)?) => {{ $(let _ = &$value;)* }};
    ($fmt:literal $(, $args:expr)* $(,)?) => {{ $(let _ = &$args;)* }};
    ($($arg:tt)*) => {{}};
}

#[macro_export]
macro_rules! error {
    ($($key:ident = $value:expr),* $(,)?) => {{ $(let _ = &$value;)* }};
    ($fmt:literal $(, $args:expr)* $(,)?) => {{ $(let _ = &$args;)* }};
    ($($arg:tt)*) => {{}};
}
