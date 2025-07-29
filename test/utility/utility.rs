pub use pretty_assertions::assert_eq;
pub use test_case::test_case;

use std::fmt::Debug;
use std::panic;

pub fn directory() -> String {
    std::env::var("TEST_UNDECLARED_OUTPUTS_DIR").unwrap_or_else(|_| "./".to_string())
}

pub fn write(filename: &str, content: &str, directory: Option<&str>) {
    let directory = directory
        .map(|d| d.to_string())
        .unwrap_or(crate::directory());
    let path = format!("{}/{}", directory, filename);

    if let Some(parent) = std::path::Path::new(&path).parent() {
        std::fs::create_dir_all(parent).unwrap();
    }

    std::fs::write(&path, content).unwrap();
    eprintln!("{}", path);
}

pub fn assert<T>(actual: T, expected: T, test: &str, name: &str)
where
    T: Debug + PartialEq + serde::Serialize,
{
    let assertion = || {
        assert_eq!(actual, expected);
    };
    println!("Artifact: ");
    write(
        &format!("{}/{}/generated.json", test, name),
        &serde_json::to_string(&actual).unwrap(),
        None,
    );
    if let Err(error) = panic::catch_unwind(panic::AssertUnwindSafe(assertion)) {
        eprintln!("Expected:");
        write(
            &format!("{}/{}/expected.json", test, name),
            &serde_json::to_string(&expected).unwrap(),
            None,
        );
        panic::resume_unwind(error);
    }
}
