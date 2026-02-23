use std::path::PathBuf;

#[must_use]
pub fn directory() -> PathBuf {
    std::env::var("BUILD_WORKING_DIRECTORY").map_or_else(|_| PathBuf::from("./"), PathBuf::from)
}
