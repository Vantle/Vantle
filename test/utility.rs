use pretty_assertions::assert_eq;
use record as _;
use std::fmt::Debug;
use std::panic;
use std::path::Path;

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

pub fn write<P: AsRef<Path>>(filename: &str, content: &str, directory: Option<P>) {
    let directory = directory.map_or_else(platform::directory, |d| d.as_ref().to_path_buf());
    let path = directory.join(filename);

    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).unwrap();
    }

    std::fs::write(&path, content).unwrap();
    eprintln!("{path}", path = path.display());
}

pub fn assert<T>(actual: &T, expected: &T, test: &str, name: &str)
where
    T: Debug + PartialEq + serde::Serialize,
{
    let assertion = || {
        assert_eq!(actual, expected);
    };
    println!("Artifact:");
    write(
        &format!("{test}/{name}/generated.json"),
        &serde_json::to_string(&actual).unwrap(),
        Option::<&str>::None,
    );
    if let Err(error) = panic::catch_unwind(panic::AssertUnwindSafe(assertion)) {
        eprintln!("Expected:");
        write(
            &format!("{test}/{name}/expected.json"),
            &serde_json::to_string(&expected).unwrap(),
            Option::<&str>::None,
        );
        panic::resume_unwind(error);
    }
}
