pub use argument;
pub use endpoint;
pub use http;
pub use observe;
pub use record;
pub use trace;

use std::collections::BTreeMap;

use endpoint::Sink;
use tokio_util::sync::CancellationToken;

fn identity(address: &str) -> String {
    address.split('?').next().unwrap_or(address).to_string()
}

pub fn initialize(sinks: &[String]) -> miette::Result<trace::Guard> {
    let mut unique = BTreeMap::new();
    for sink in sinks {
        let normalized = endpoint::normalize(sink);
        unique.insert(identity(&normalized), normalized);
    }
    let resolved = unique
        .into_values()
        .map(|a| endpoint::resolve(&a))
        .collect::<error::Result<Vec<_>>>()
        .map_err(|e| miette::Report::new(*e))
        .map(|r| {
            if r.is_empty() {
                vec![endpoint::stdout()]
            } else {
                r
            }
        })?;

    let (traces, urls): (Vec<_>, Vec<_>) = resolved
        .into_iter()
        .partition(|s| !matches!(s, Sink::Http(_)));

    let traces = Some(traces)
        .filter(|t| !t.is_empty())
        .unwrap_or_else(|| vec![endpoint::stdout()]);

    let cancellation = CancellationToken::new();

    let mut guard =
        trace::initialize(traces, cancellation.clone()).map_err(|e| miette::Report::new(*e))?;

    for url in urls.into_iter().filter_map(|s| match s {
        Sink::Http(stream) => Some(stream.url),
        _ => None,
    }) {
        let handle = http::spawn(url, cancellation.clone()).map_err(|e| miette::Report::new(*e))?;
        guard.track(handle);
    }

    Ok(guard)
}
