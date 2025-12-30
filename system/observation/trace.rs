use std::path::PathBuf;
use std::sync::{Mutex, PoisonError};
use url::Url;

pub enum Sink {
    Stdout,
    File(PathBuf),
}

pub fn resolve(address: Option<&str>) -> error::Result<Sink> {
    let Some(address) = address else {
        return Ok(Sink::Stdout);
    };

    let url = Url::parse(address).map_err(|source| error::Error::Parse {
        address: address.to_string(),
        source,
    })?;

    match url.scheme() {
        "file" => Ok(Sink::File(PathBuf::from(url.path()))),
        "grpc" => Err(error::Error::Scheme {
            scheme: "grpc (use --config=stream for gRPC support)".to_string(),
        }),
        scheme => Err(error::Error::Scheme {
            scheme: scheme.to_string(),
        }),
    }
}

#[must_use]
pub fn default() -> PathBuf {
    platform::directory().join("trace.json")
}

#[inline]
pub fn store<T>(guard: &Mutex<Option<T>>, value: T) {
    *guard.lock().unwrap_or_else(PoisonError::into_inner) = Some(value);
}

#[inline]
pub fn clear<T>(guard: &Mutex<Option<T>>) {
    guard.lock().unwrap_or_else(PoisonError::into_inner).take();
}
