use tracing::level_filters::LevelFilter;

#[cfg(level_trace)]
pub const LEVEL: LevelFilter = LevelFilter::TRACE;

#[cfg(level_debug)]
pub const LEVEL: LevelFilter = LevelFilter::DEBUG;

#[cfg(level_info)]
pub const LEVEL: LevelFilter = LevelFilter::INFO;

#[cfg(level_warn)]
pub const LEVEL: LevelFilter = LevelFilter::WARN;

#[cfg(level_error)]
pub const LEVEL: LevelFilter = LevelFilter::ERROR;

#[cfg(not(any(level_trace, level_debug, level_info, level_warn, level_error)))]
pub const LEVEL: LevelFilter = LevelFilter::INFO;
