use std::path::{Path, PathBuf};

use miette::Diagnostic;
use thiserror::Error;

#[derive(Error, Debug, Diagnostic)]
pub enum Error {
    #[error("sandbox path does not contain a /bin/ segment: {path}")]
    #[diagnostic(
        code(bazel::symlink::segment),
        help(
            "expected a Bazel output path containing /bin/, e.g. /private/var/tmp/.../bin/pkg/file"
        )
    )]
    Segment { path: String },
}

pub fn resolve(sandbox: &Path, prefix: &str) -> miette::Result<PathBuf> {
    let display = sandbox.display().to_string();
    let index = display.find("/bin/").ok_or_else(|| Error::Segment {
        path: display.clone(),
    })?;
    Ok(PathBuf::from(format!(
        "{}bin/{}",
        prefix,
        &display[index + 5..]
    )))
}
