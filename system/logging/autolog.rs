pub use logging::{debug, error, info, warn};

#[ctor::ctor]
fn initialize() {
    #[cfg(debug_assertions)]
    let level = log::LevelFilter::Debug;

    #[cfg(not(debug_assertions))]
    let level = log::LevelFilter::Error;

    logging::configure(level);
}
