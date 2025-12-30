use std::marker::PhantomData;

pub struct Guard(PhantomData<()>);

pub use category;

pub mod serialize {
    use std::fmt;
    use std::marker::PhantomData;

    pub struct Json<'a, T: ?Sized>(PhantomData<&'a T>);

    impl<T: ?Sized> fmt::Display for Json<'_, T> {
        fn fmt(&self, _: &mut fmt::Formatter<'_>) -> fmt::Result {
            Ok(())
        }
    }

    impl<T: ?Sized> fmt::Debug for Json<'_, T> {
        fn fmt(&self, _: &mut fmt::Formatter<'_>) -> fmt::Result {
            Ok(())
        }
    }

    #[must_use]
    #[inline]
    pub fn json<T: ?Sized>(_: &T) -> Json<'_, T> {
        Json(PhantomData)
    }
}

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
