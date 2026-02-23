use std::path::PathBuf;

#[must_use]
pub fn output() -> PathBuf {
    std::env::var("TEST_UNDECLARED_OUTPUTS_DIR").map_or_else(|_| PathBuf::from("./"), PathBuf::from)
}
