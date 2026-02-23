use std::path::PathBuf;

#[must_use]
pub fn directory() -> Option<PathBuf> {
    std::env::current_dir().ok()
}
