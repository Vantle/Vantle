use std::fs::File;
use std::io::LineWriter;
use std::path::Path;
use std::sync::Mutex;

use tracing_subscriber::fmt;
use tracing_subscriber::registry::LookupSpan;

pub fn file<S>(
    path: &Path,
    ansi: bool,
) -> error::Result<
    fmt::Layer<S, fmt::format::DefaultFields, fmt::format::Format, Mutex<LineWriter<File>>>,
>
where
    S: tracing::Subscriber + for<'lookup> LookupSpan<'lookup>,
{
    let file = File::create(path).map_err(|source| error::Error::File {
        path: path.to_string_lossy().into_owned(),
        source,
    })?;

    Ok(fmt::layer()
        .with_ansi(ansi)
        .with_writer(Mutex::new(LineWriter::new(file))))
}
