pub use argument;
pub use endpoint;
pub use http;
pub use observe;
pub use record;
pub use trace;

use endpoint::Sink;
use tokio_util::sync::CancellationToken;

fn report<T>(result: error::Result<T>) -> miette::Result<T> {
    result.map_err(|e| miette::Report::new(*e))
}

pub fn initialize(sinks: &[String]) -> miette::Result<trace::Guard> {
    let resolved = sinks
        .iter()
        .map(|s| endpoint::resolve(&endpoint::normalize(s)))
        .collect::<error::Result<Vec<_>>>();
    let resolved = report(resolved)?;

    let (traces, urls): (Vec<_>, Vec<_>) = resolved
        .into_iter()
        .partition(|s| !matches!(s, Sink::Http(_)));

    let traces = Some(traces)
        .filter(|t| !t.is_empty())
        .unwrap_or_else(|| vec![endpoint::stdout()]);

    let cancellation = CancellationToken::new();

    let mut guard = report(trace::initialize(traces, cancellation.clone()))?;

    for url in urls.into_iter().filter_map(|s| match s {
        Sink::Http(stream) => Some(stream.url),
        _ => None,
    }) {
        let handle = report(http::spawn(url, cancellation.clone()))?;
        guard.track(handle);
    }

    Ok(guard)
}
