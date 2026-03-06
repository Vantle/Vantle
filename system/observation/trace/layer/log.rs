use std::fs::File;
use std::io::{LineWriter, Write};
use std::path::Path;
use std::sync::Mutex;

use tracing_subscriber::fmt;
use tracing_subscriber::registry::LookupSpan;

type Writer = Mutex<LineWriter<Box<dyn Write + Send>>>;

#[must_use]
pub fn stdout<S>(
    ansi: bool,
) -> fmt::Layer<S, fmt::format::DefaultFields, fmt::format::Format, Writer>
where
    S: tracing::Subscriber + for<'lookup> LookupSpan<'lookup>,
{
    let writer: Box<dyn Write + Send> = Box::new(std::io::stdout());
    fmt::layer()
        .with_ansi(ansi)
        .with_writer(Mutex::new(LineWriter::new(writer)))
}

#[must_use]
pub fn stderr<S>(
    ansi: bool,
) -> fmt::Layer<S, fmt::format::DefaultFields, fmt::format::Format, Writer>
where
    S: tracing::Subscriber + for<'lookup> LookupSpan<'lookup>,
{
    let writer: Box<dyn Write + Send> = Box::new(std::io::stderr());
    fmt::layer()
        .with_ansi(ansi)
        .with_writer(Mutex::new(LineWriter::new(writer)))
}

pub fn file<S>(
    path: &Path,
    ansi: bool,
) -> error::Result<fmt::Layer<S, fmt::format::DefaultFields, fmt::format::Format, Writer>>
where
    S: tracing::Subscriber + for<'lookup> LookupSpan<'lookup>,
{
    let file = File::create(path).map_err(|source| error::Error::File {
        path: path.to_string_lossy().into_owned(),
        source,
    })?;

    let writer: Box<dyn Write + Send> = Box::new(file);
    Ok(fmt::layer()
        .with_ansi(ansi)
        .with_writer(Mutex::new(LineWriter::new(writer))))
}
