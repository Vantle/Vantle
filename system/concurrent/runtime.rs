pub use error;

use std::sync::OnceLock;
use tokio::runtime::{Builder, Runtime};
use tokio_util::sync::CancellationToken;

static RUNTIME: OnceLock<Runtime> = OnceLock::new();
static CANCELLATION: OnceLock<CancellationToken> = OnceLock::new();

pub fn global() -> error::Result<&'static Runtime> {
    if let Some(runtime) = RUNTIME.get() {
        return Ok(runtime);
    }

    let runtime = Builder::new_multi_thread()
        .worker_threads(64)
        .thread_name("concurrent")
        .enable_all()
        .build()
        .map_err(|source| error::Error::Runtime { source })?;

    Ok(RUNTIME.get_or_init(|| runtime))
}

pub fn token() -> &'static CancellationToken {
    CANCELLATION.get_or_init(CancellationToken::new)
}

pub fn shutdown() {
    if let Some(t) = CANCELLATION.get() {
        t.cancel();
    }
}
