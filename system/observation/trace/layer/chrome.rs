use std::path::Path;

use tracing_chrome::{ChromeLayer, ChromeLayerBuilder, FlushGuard};
use tracing_subscriber::registry::LookupSpan;

pub fn layer<S>(path: &Path) -> error::Result<(ChromeLayer<S>, FlushGuard)>
where
    S: tracing::Subscriber + for<'lookup> LookupSpan<'lookup> + Send + Sync,
{
    let (layer, guard) = ChromeLayerBuilder::new().file(path).build();
    Ok((layer, guard))
}
