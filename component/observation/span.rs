pub type Guard = tracing::span::EnteredSpan;

#[macro_export]
macro_rules! enter {
    ($name:expr) => {
        tracing::info_span!($name).entered()
    };
}
