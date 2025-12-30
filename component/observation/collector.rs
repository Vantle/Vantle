pub use tracing;
pub use tracing_subscriber;

#[macro_export]
macro_rules! event {
    ($($arg:tt)*) => {
        tracing::info!($($arg)*)
    };
}
