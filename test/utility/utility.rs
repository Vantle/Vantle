//! Assertion utilities for test generation.
//!
//! This module provides enhanced assertion functions that dump useful artifacts
//! when tests fail, making debugging easier.

pub use pretty_assertions::assert_eq;
pub use test_case::test_case;

use serde_json::to_string_pretty;
use std::fmt::Debug;
use std::panic;

/// Write content to a file in the test output directory and print the path.
fn write(filename: &str, content: &str, directory: Option<&str>) {
    let directory = directory.map(|d| d.to_string()).unwrap_or_else(|| {
        std::env::var("TEST_UNDECLARED_OUTPUTS_DIR").unwrap_or_else(|_| "./".to_string())
    });
    let path = format!("{}/{}", directory, filename);

    // Create parent directories if they don't exist
    if let Some(parent) = std::path::Path::new(&path).parent() {
        std::fs::create_dir_all(parent).unwrap();
    }

    std::fs::write(&path, content).unwrap();
    eprintln!("{}", path);
}

/// Enhanced assertion that dumps artifacts on failure.
///
/// When the assertion fails, this function:
/// 1. Writes the actual value as pretty JSON to `{test}/{name}/actual.json`
/// 2. Writes the expected value as pretty JSON to `{test}/{name}/expected.json`
/// 3. Re-raises the panic so the test still fails
pub fn assert<T>(actual: T, expected: T, test: &str, name: &str)
where
    T: Debug + PartialEq + serde::Serialize,
{
    let assertion = || {
        assert_eq!(actual, expected);
    };

    if let Err(error) = panic::catch_unwind(panic::AssertUnwindSafe(assertion)) {
        eprintln!("Artifacts:");
        write(
            &format!("{}/{}/actual.json", test, name),
            &to_string_pretty(&actual).unwrap(),
            None,
        );
        write(
            &format!("{}/{}/expected.json", test, name),
            &to_string_pretty(&expected).unwrap(),
            None,
        );
        panic::resume_unwind(error);
    }
}
