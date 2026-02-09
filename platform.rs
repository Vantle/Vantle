use std::path::PathBuf;

use miette::Diagnostic;
use thiserror::Error;

#[derive(Error, Debug, Diagnostic)]
pub enum Error {
    #[error("Missing Bazel environment")]
    #[diagnostic(code(platform::bazel), help("Run via `bazel run` or `bazel test`"))]
    Bazel,
}

pub type Result<T> = std::result::Result<T, Error>;

pub fn directory() -> PathBuf {
    std::env::var("TEST_UNDECLARED_OUTPUTS_DIR")
        .or_else(|_| std::env::var("BUILD_WORKING_DIRECTORY"))
        .map_or_else(|_| PathBuf::from("./"), PathBuf::from)
}
