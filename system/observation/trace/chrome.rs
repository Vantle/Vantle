pub use channel;
pub use error;

use channel::Channel;
use filter::Filter;
use std::path::PathBuf;
use std::sync::Mutex;
use trace::Sink;
use tracing_chrome::{ChromeLayerBuilder, FlushGuard};
use tracing_subscriber::prelude::*;

static GUARD: Mutex<Option<FlushGuard>> = Mutex::new(None);
static PATH: Mutex<Option<PathBuf>> = Mutex::new(None);

pub fn initialize<F>(address: Option<&str>, predicate: F) -> error::Result<()>
where
    F: Fn(&[Channel]) -> bool + Send + Sync + 'static,
{
    let path = match trace::resolve(address)? {
        Sink::File(path) => path,
        Sink::Stdout => trace::default(),
    };

    let (chrome, guard) = ChromeLayerBuilder::new().file(&path).build();

    tracing_subscriber::registry()
        .with(Filter::new(predicate))
        .with(
            tracing_subscriber::fmt::layer()
                .with_ansi(true)
                .with_filter(configuration::LEVEL),
        )
        .with(chrome.with_filter(configuration::LEVEL))
        .try_init()
        .map_err(|e| error::Error::Subscriber {
            details: e.to_string(),
        })?;

    trace::store(&GUARD, guard);
    trace::store(&PATH, path);
    Ok(())
}

#[must_use]
pub fn path() -> Option<String> {
    PATH.lock()
        .ok()
        .and_then(|p| p.as_ref().map(|p| p.to_string_lossy().into_owned()))
}

pub fn flush() {
    trace::clear(&GUARD);
}
