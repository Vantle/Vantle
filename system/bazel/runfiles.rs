use std::path::{Path, PathBuf};

use miette::Diagnostic;
use thiserror::Error;

#[derive(Error, Debug, Diagnostic)]
pub enum Error {
    #[error("failed to determine executable path")]
    #[diagnostic(
        code(bazel::runfiles::executable),
        help("check that the binary is invoked correctly")
    )]
    Executable,

    #[error("runfiles directory not found for {executable}")]
    #[diagnostic(
        code(bazel::runfiles::missing),
        help("ensure the binary is run via bazel run or has a .runfiles directory")
    )]
    Missing { executable: String },

    #[error("no runfile matching suffix '{suffix}'")]
    #[diagnostic(
        code(bazel::runfiles::lookup),
        help("check that the expected file is included in the rule's runfiles")
    )]
    Lookup { suffix: String },
}

fn executable() -> miette::Result<(PathBuf, String)> {
    let path = std::env::current_exe().map_err(|_| Error::Executable)?;
    let name = path
        .file_name()
        .ok_or(Error::Executable)?
        .to_string_lossy()
        .into_owned();
    Ok((path, name))
}

pub fn discover() -> miette::Result<PathBuf> {
    let (path, name) = executable()?;

    let runfiles = path
        .with_file_name(format!("{name}.runfiles"))
        .join("_main");
    if runfiles.exists() {
        return Ok(runfiles);
    }

    if let Ok(directory) = std::env::var("RUNFILES_DIR") {
        let path = PathBuf::from(directory);
        if path.exists() {
            return Ok(path);
        }
    }

    Err(Error::Missing { executable: name }.into())
}

pub fn find(suffix: &str) -> miette::Result<PathBuf> {
    let (path, name) = executable()?;

    let manifest = path.with_file_name(format!("{name}.runfiles_manifest"));
    if let Ok(content) = std::fs::read_to_string(&manifest) {
        for line in content.lines() {
            let mut parts = line.splitn(2, ' ');
            if let (Some(logical), Some(physical)) = (parts.next(), parts.next())
                && logical.ends_with(suffix)
            {
                return Ok(PathBuf::from(physical));
            }
        }
    }

    let runfiles = discover()?;
    scan(&runfiles, suffix)
}

fn scan(directory: &Path, suffix: &str) -> miette::Result<PathBuf> {
    if let Ok(entries) = std::fs::read_dir(directory) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() && path.to_string_lossy().ends_with(suffix) {
                return Ok(path);
            }
            if path.is_dir()
                && let Ok(found) = scan(&path, suffix)
            {
                return Ok(found);
            }
        }
    }
    Err(Error::Lookup {
        suffix: suffix.to_owned(),
    }
    .into())
}
