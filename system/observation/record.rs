pub use error;

use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::Path;

use stream::{Record, Update};

pub fn write<P: AsRef<Path>>(path: P, updates: &[Update]) -> error::Result<Record> {
    let location = path.as_ref().to_string_lossy().into_owned();

    let file = File::create(path.as_ref()).map_err(|source| error::Error::Create {
        path: location.clone(),
        source,
    })?;

    let mut writer = BufWriter::new(file);

    let count = updates.len() as u64;

    let stamp = |u: &Update| match u {
        Update::Span(s) => match &s.lifecycle {
            stream::Lifecycle::Begin(b) => b.timestamp,
            stream::Lifecycle::End(e) => e.timestamp,
        },
        Update::Event(e) => e.timestamp,
        Update::Snapshot(s) => s.timestamp,
    };

    let duration = updates
        .first()
        .zip(updates.last())
        .map_or(0, |(first, last)| stamp(last).saturating_sub(stamp(first)));

    let serialized = serde_json::to_vec(updates).map_err(|source| error::Error::Format {
        path: location.clone(),
        source,
    })?;

    writer
        .write_all(&serialized)
        .map_err(|source| error::Error::Write {
            path: location.clone(),
            source,
        })?;

    writer.flush().map_err(|source| error::Error::Write {
        path: location.clone(),
        source,
    })?;

    Ok(Record {
        path: location,
        count,
        duration,
    })
}

pub fn read<P: AsRef<Path>>(path: P) -> error::Result<Vec<Update>> {
    let location = path.as_ref().to_string_lossy().into_owned();

    let file = File::open(path).map_err(|source| error::Error::Open {
        path: location.clone(),
        source,
    })?;

    let mut reader = BufReader::new(file);
    let mut contents = Vec::new();

    reader
        .read_to_end(&mut contents)
        .map_err(|source| error::Error::Read {
            path: location.clone(),
            source,
        })?;

    serde_json::from_slice(&contents).map_err(|source| error::Error::Format {
        path: location,
        source,
    })
}
