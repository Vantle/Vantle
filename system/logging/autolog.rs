pub use logging::{debug, error, info, warn};

#[ctor::ctor]
fn initialize() {
    logging::configure(log::LevelFilter::Debug);
}
