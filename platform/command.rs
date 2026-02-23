use std::path::PathBuf;

use miette::{Diagnostic, Result};
use thiserror::Error;

pub const FILE: &str = "arguments";

#[derive(Error, Debug, Diagnostic)]
pub enum Error {
    #[error("failed to read arguments from {path}")]
    #[diagnostic(
        code(platform::command::arguments),
        help("check that the arguments file exists and is readable")
    )]
    Read {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
}

#[must_use]
pub fn path() -> PathBuf {
    run::directory().map_or_else(|| PathBuf::from(FILE), |d| d.join(FILE))
}

pub fn arguments() -> Result<Vec<String>> {
    let path = path();
    match std::fs::read_to_string(&path) {
        Ok(content) => Ok(std::iter::once(String::new())
            .chain(content.lines().filter(|l| !l.is_empty()).map(String::from))
            .collect::<Vec<_>>()),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            Ok(std::env::args().collect::<Vec<_>>())
        }
        Err(source) => Err(Error::Read { path, source }.into()),
    }
}
