pub use error;
pub use execution;

use std::collections::HashMap;
use std::path::PathBuf;

use clap::Parser;
use concurrent::Join;
use observe::trace;
use serde::Serialize;
use serde_json::{Map, Value};

use error::Error;
use execution::{Caught, Execution, Meta, Source, Task};

fn destination() -> PathBuf {
    test::output()
        .unwrap_or_default()
        .join("cases.execution.json")
}

#[derive(Parser)]
#[command(name = "executor")]
#[command(about = "Execute test cases and produce a structured report")]
pub struct Arguments {
    #[arg(long, default_value_os_t = destination())]
    pub output: PathBuf,
    #[command(flatten)]
    pub sink: argument::Argument,
}

pub type Evaluation = Result<Map<String, Value>, Box<Error>>;

#[derive(Default, derive_more::Into)]
pub struct Actuals(Map<String, Value>);

impl Actuals {
    pub fn record(mut self, key: &str, value: &impl Serialize) -> Result<Self, Box<Error>> {
        let json = serde_json::to_value(value)
            .map_err(|cause| Box::new(Error::serialization(key, cause)))?;
        self.0.insert(key.to_string(), json);
        Ok(self)
    }
}

pub struct Executor {
    arguments: Arguments,
    source: Source,
    tasks: Vec<Box<dyn FnOnce() -> Task + Send>>,
    indices: HashMap<String, usize>,
}

impl Executor {
    pub fn new(arguments: Arguments, file: impl Into<PathBuf>, cases: impl Into<PathBuf>) -> Self {
        let source = Source {
            file: file.into(),
            cases: cases.into(),
        };
        Self {
            arguments,
            source,
            tasks: Vec::new(),
            indices: HashMap::new(),
        }
    }

    pub fn register(
        &mut self,
        name: &str,
        tags: &[&str],
        parameters: Value,
        expected: Value,
        function: impl FnOnce() -> Evaluation + Send + 'static,
    ) {
        let index = self.indices.entry(name.to_string()).or_insert(0);
        let meta = Meta::new(name, tags, *index);
        *index += 1;
        self.tasks.push(Box::new(move || {
            let result: Caught = std::panic::catch_unwind(std::panic::AssertUnwindSafe(function));
            if let Ok(Ok(_)) = &result {
                tracing::info!("✓ {}", meta.identifier);
            } else {
                tracing::error!("✗ {}", meta.identifier);
            }
            Task {
                meta,
                parameters,
                expected,
                result,
            }
        }));
    }

    #[trace(channels = [test])]
    pub fn wait(self, schedule: &impl concurrent::Schedule) -> miette::Result<()> {
        let tasks = self
            .tasks
            .into_iter()
            .map(|task| schedule.execute(move || Execution::resolve(task())))
            .collect::<Vec<_>>();

        let executions = schedule.block(async {
            let mut results = Vec::new();
            for task in tasks {
                if let Ok(execution) = task.join().await {
                    results.push(execution);
                }
            }
            results
        });

        let failures =
            execution::write(self.source, executions, &self.arguments.output).map_err(|e| *e)?;
        if !failures.is_empty() {
            return Err(Error::collection(failures, &self.arguments.output).into());
        }
        Ok(())
    }
}
