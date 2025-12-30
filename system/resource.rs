use std::fs::File;
use std::io::{self, Cursor, Read};
use std::path::Path;

use miette::Diagnostic;
use thiserror::Error;

pub type Result<T> = miette::Result<T, Error>;

#[derive(Error, Debug, Diagnostic)]
#[error(transparent)]
#[diagnostic(code(resource::io_error))]
pub struct Error(#[from] pub io::Error);

pub fn read(path: impl AsRef<Path>) -> Result<File> {
    let path = path.as_ref();
    Ok(File::options().read(true).open(path)?)
}

pub fn stringify(path: impl AsRef<Path>) -> Result<String> {
    let mut file = read(path)?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;
    Ok(buffer)
}

pub fn cursor(resource: impl AsRef<Path>) -> Result<Cursor<Vec<u8>>> {
    let resource = resource.as_ref();
    let buffer = stringify(resource)?;
    Ok(Cursor::new(buffer.as_bytes().into()))
}
