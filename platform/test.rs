use std::path::PathBuf;

#[must_use]
pub fn output() -> Option<PathBuf> {
    std::env::var("TEST_UNDECLARED_OUTPUTS_DIR")
        .ok()
        .map(PathBuf::from)
}
