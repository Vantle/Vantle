use tracing::level_filters::LevelFilter;

#[cfg(trace)]
pub const LEVEL: LevelFilter = LevelFilter::TRACE;

#[cfg(debug)]
pub const LEVEL: LevelFilter = LevelFilter::DEBUG;

#[cfg(info)]
pub const LEVEL: LevelFilter = LevelFilter::INFO;

#[cfg(warn)]
pub const LEVEL: LevelFilter = LevelFilter::WARN;

#[cfg(error)]
pub const LEVEL: LevelFilter = LevelFilter::ERROR;

#[cfg(not(any(trace, debug, info, warn, error)))]
pub const LEVEL: LevelFilter = LevelFilter::INFO;
