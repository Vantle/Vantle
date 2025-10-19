pub use pretty_assertions::assert_eq;
pub use test_case::test_case;

use std::fmt::Debug;
use std::panic;
use std::path::{Path, PathBuf};

pub fn unwrap<T, E>(result: Result<T, E>) -> T
where
    E: miette::Diagnostic + std::error::Error + Send + Sync + 'static,
{
    match result {
        Ok(value) => value,
        Err(error) => {
            eprintln!("{:?}", miette::Report::new(error));
            panic!("expected Ok result");
        }
    }
}

pub fn directory() -> PathBuf {
    std::env::var("TEST_UNDECLARED_OUTPUTS_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("./"))
}

pub fn write<P: AsRef<Path>>(filename: &str, content: &str, directory: Option<P>) {
    let directory = directory
        .map(|d| d.as_ref().to_path_buf())
        .unwrap_or_else(crate::directory);
    let path = directory.join(filename);

    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).unwrap();
    }

    std::fs::write(&path, content).unwrap();
    eprintln!("{}", path.display());
}

pub fn assert<T>(actual: T, expected: T, test: impl Into<String>, name: impl Into<String>)
where
    T: Debug + PartialEq + serde::Serialize,
{
    let test = test.into();
    let name = name.into();
    let assertion = || {
        assert_eq!(actual, expected);
    };
    println!("Artifact:");
    write(
        &format!("{}/{}/generated.json", test, name),
        &serde_json::to_string(&actual).unwrap(),
        Option::<&str>::None,
    );
    if let Err(error) = panic::catch_unwind(panic::AssertUnwindSafe(assertion)) {
        eprintln!("Expected:");
        write(
            &format!("{}/{}/expected.json", test, name),
            &serde_json::to_string(&expected).unwrap(),
            Option::<&str>::None,
        );
        panic::resume_unwind(error);
    }
}
