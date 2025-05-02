pub use log::{debug, error, info, warn};

#[ctor::ctor]
fn initialize() {
    simple_logger::SimpleLogger::new()
        .with_colors(true)
        .with_level(log::LevelFilter::Debug)
        .init()
        .expect("Failed to initialize logging");
}
