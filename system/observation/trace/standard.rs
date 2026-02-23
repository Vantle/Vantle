pub use channel;
pub use error;

use channel::Channel;
use filter::Filter;
use std::fs::File;
use std::io::LineWriter;
use std::path::PathBuf;
use std::sync::Mutex;
use trace::Sink;
use tracing_subscriber::prelude::*;

static PATH: Mutex<Option<PathBuf>> = Mutex::new(None);

pub fn initialize<F>(address: Option<&str>, predicate: F) -> error::Result<()>
where
    F: Fn(&[Channel]) -> bool + Send + Sync + 'static,
{
    let filter = Filter::new(predicate);
    match trace::resolve(address)? {
        Sink::File(path) => {
            let file = File::create(&path).map_err(|source| error::Error::File {
                path: path.to_string_lossy().into_owned(),
                source,
            })?;
            tracing_subscriber::registry()
                .with(filter)
                .with(
                    tracing_subscriber::fmt::layer()
                        .with_ansi(false)
                        .with_writer(Mutex::new(LineWriter::new(file)))
                        .with_filter(configuration::LEVEL),
                )
                .try_init()
                .map_err(|e| error::Error::Subscriber {
                    details: e.to_string(),
                })?;
            trace::store(&PATH, path);
            preamble();
            Ok(())
        }
        Sink::Stdout => tracing_subscriber::registry()
            .with(filter)
            .with(
                tracing_subscriber::fmt::layer()
                    .with_ansi(true)
                    .with_filter(configuration::LEVEL),
            )
            .try_init()
            .map_err(|e| error::Error::Subscriber {
                details: e.to_string(),
            }),
    }
}

fn preamble() {
    if let Some(path) = path() {
        let _span = tracing::info_span!("initialize").entered();
        tracing::info!("{path}");
    }
}

#[inline]
#[must_use]
pub fn path() -> Option<String> {
    PATH.lock()
        .ok()
        .and_then(|g| g.as_ref().map(|p| p.to_string_lossy().into_owned()))
}

pub fn flush() {
    postamble();
}

fn postamble() {
    if let Some(path) = path() {
        let _span = tracing::info_span!("flush").entered();
        tracing::info!("{path}");
    }
}
