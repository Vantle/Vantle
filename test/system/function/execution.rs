use std::io::Write;
use std::path::{Path, PathBuf};

use serde::Serialize;
use serde_json::{Map, Value};

use error::Error;

#[derive(Serialize)]
pub struct Source {
    pub file: PathBuf,
    pub cases: PathBuf,
}

pub struct Meta {
    pub name: String,
    pub identifier: String,
    pub tags: Vec<String>,
}

impl Meta {
    #[must_use]
    pub fn new(name: &str, tags: &[&str], index: usize) -> Self {
        let identifier = if tags.is_empty() {
            format!("{name}.{index}")
        } else {
            let joined = tags.join(".");
            format!("{name}.{joined}.{index}")
        };
        Self {
            name: name.to_string(),
            identifier,
            tags: tags.iter().map(|s| (*s).to_string()).collect::<Vec<_>>(),
        }
    }
}

pub enum Outcome {
    Pass,
    Mismatch(Map<String, Value>),
    Panic(String),
}

pub type Caught =
    std::result::Result<Result<Map<String, Value>, Box<Error>>, Box<dyn std::any::Any + Send>>;

pub struct Task {
    pub meta: Meta,
    pub parameters: Value,
    pub expected: Value,
    pub result: Caught,
}

pub struct Execution {
    pub meta: Meta,
    pub parameters: Value,
    pub expected: Value,
    pub outcome: Outcome,
}

impl Execution {
    #[must_use]
    pub fn resolve(task: Task) -> Self {
        let outcome = match task.result {
            Ok(Ok(_)) => Outcome::Pass,
            Ok(Err(error)) => match *error {
                Error::Mismatch { actuals, .. } => Outcome::Mismatch(actuals),
                other => Outcome::Panic(format!("{:?}", miette::Report::new(other))),
            },
            Err(panic) => {
                let message = panic
                    .downcast_ref::<String>()
                    .map(String::as_str)
                    .or_else(|| panic.downcast_ref::<&str>().copied())
                    .unwrap_or("unknown panic")
                    .to_string();
                Outcome::Panic(message)
            }
        };
        Self {
            meta: task.meta,
            parameters: task.parameters,
            expected: task.expected,
            outcome,
        }
    }
}

#[derive(Serialize)]
struct Document {
    source: Source,
    functions: Vec<Group>,
}

#[derive(Serialize)]
struct Group {
    function: String,
    tags: Vec<String>,
    cases: Vec<Case>,
}

#[derive(Serialize)]
struct Case {
    parameters: Value,
    returns: Value,
    unexpected: Option<Value>,
}

pub fn write(
    source: Source,
    mut executions: Vec<Execution>,
    destination: &Path,
) -> Result<Vec<error::Failure>, Box<Error>> {
    executions.sort_by(|a, b| a.meta.name.cmp(&b.meta.name));

    let mut functions: Vec<Group> = Vec::new();
    let mut failures = Vec::<error::Failure>::new();

    for execution in executions.drain(..) {
        let unexpected = match execution.outcome {
            Outcome::Pass => None,
            Outcome::Mismatch(ref actuals) => {
                let actual = serde_json::to_string_pretty(actuals).unwrap_or_default();
                let expected =
                    serde_json::to_string_pretty(&execution.expected).unwrap_or_default();
                failures.push(error::Failure::new(
                    execution.meta.identifier.clone(),
                    format!("expected:\n{expected}\n\nactual:\n{actual}"),
                ));
                Some(Value::Object(actuals.clone()))
            }
            Outcome::Panic(ref message) => {
                failures.push(error::Failure::new(
                    execution.meta.identifier.clone(),
                    message.clone(),
                ));
                let mut map = Map::new();
                map.insert("panic".to_string(), Value::String(message.clone()));
                Some(Value::Object(map))
            }
        };

        let case = Case {
            parameters: execution.parameters,
            returns: execution.expected,
            unexpected,
        };

        match functions
            .last_mut()
            .filter(|f| f.function == execution.meta.name)
        {
            Some(function) => function.cases.push(case),
            None => functions.push(Group {
                function: execution.meta.name,
                tags: execution.meta.tags,
                cases: vec![case],
            }),
        }
    }

    let document = Document { source, functions };

    let json = serde_json::to_string_pretty(&document)
        .map_err(|cause| Box::new(Error::serialization("report", cause)))?;

    if let Some(parent) = destination.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|cause| Box::new(Error::write(destination, cause)))?;
    }

    let temporary = destination.with_extension("tmp");
    let mut file = std::fs::File::create(&temporary)
        .map_err(|cause| Box::new(Error::write(&temporary, cause)))?;
    file.write_all(json.as_bytes())
        .map_err(|cause| Box::new(Error::write(&temporary, cause)))?;
    file.sync_all()
        .map_err(|cause| Box::new(Error::write(&temporary, cause)))?;
    drop(file);
    std::fs::rename(&temporary, destination)
        .map_err(|cause| Box::new(Error::write(destination, cause)))?;

    Ok(failures)
}
