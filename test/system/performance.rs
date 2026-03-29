pub use error;

use std::collections::HashMap;
use std::io::Write;
use std::path::PathBuf;

use clap::Parser;
use serde::Serialize;

use error::Error;

fn destination() -> PathBuf {
    test::output()
        .unwrap_or_default()
        .join("performance.cases.execution.json")
}

#[derive(Parser)]
#[command(name = "performance")]
#[command(about = "Execute performance cases and produce a structured report")]
pub struct Arguments {
    #[arg(long, default_value_os_t = destination())]
    pub output: PathBuf,
    #[command(flatten)]
    pub sink: argument::Argument,
    #[arg(long, default_value = "off")]
    pub sample: Toggle,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Toggle {
    On,
    Off,
}

impl Toggle {
    #[must_use]
    pub fn enabled(self) -> bool {
        matches!(self, Self::On)
    }
}

impl std::str::FromStr for Toggle {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "on" => Ok(Self::On),
            "off" => Ok(Self::Off),
            other => Err(format!("expected 'on' or 'off', got '{other}'")),
        }
    }
}

pub struct Timing {
    pub point: Vec<f64>,
    pub observation: f64,
}

pub struct Measured {
    pub name: String,
    pub tags: Vec<String>,
    pub dimensions: Vec<String>,
    pub timings: Vec<Timing>,
}

#[must_use]
#[expect(clippy::cast_precision_loss)]
pub fn dimension(value: usize) -> f64 {
    value as f64
}

pub struct Sampler {
    arguments: Arguments,
    source: Source,
    functions: Vec<Measured>,
}

#[derive(Serialize)]
struct Source {
    template: PathBuf,
    cases: PathBuf,
    specification: PathBuf,
}

impl Sampler {
    #[must_use]
    pub fn new(
        arguments: Arguments,
        template: impl Into<PathBuf>,
        cases: impl Into<PathBuf>,
        specification: impl Into<PathBuf>,
    ) -> Self {
        Self {
            arguments,
            source: Source {
                template: template.into(),
                cases: cases.into(),
                specification: specification.into(),
            },
            functions: Vec::new(),
        }
    }

    pub fn register(&mut self, measured: Measured) {
        self.functions.push(measured);
    }

    pub fn wait(self, _schedule: &impl concurrent::Schedule) -> miette::Result<()> {
        let mut report = Report {
            source: self.source,
            functions: Vec::new(),
        };

        for measured in self.functions {
            report
                .functions
                .push(analyze(&measured, &self.arguments, measured.tags.clone()));
        }

        let json = serde_json::to_string_pretty(&report).map_err(|e| Error::Correctness {
            help: format!("failed to serialize report: {e}"),
        })?;

        if let Some(parent) = self.arguments.output.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|cause| Error::write(&self.arguments.output, cause))?;
        }

        let temporary = self.arguments.output.with_extension("tmp");
        let mut file =
            std::fs::File::create(&temporary).map_err(|cause| Error::write(&temporary, cause))?;
        file.write_all(json.as_bytes())
            .map_err(|cause| Error::write(&temporary, cause))?;
        file.sync_all()
            .map_err(|cause| Error::write(&temporary, cause))?;
        drop(file);
        std::fs::rename(&temporary, &self.arguments.output)
            .map_err(|cause| Error::write(&self.arguments.output, cause))?;

        Ok(())
    }
}

#[expect(clippy::cast_precision_loss, clippy::cast_possible_truncation)]
fn analyze(measured: &Measured, arguments: &Arguments, tags: Vec<String>) -> Function {
    let mut grouped: HashMap<Vec<i64>, Vec<f64>> = HashMap::new();

    for timing in &measured.timings {
        let key = timing.point.iter().map(|v| *v as i64).collect::<Vec<_>>();
        grouped.entry(key).or_default().push(timing.observation);
    }

    let mut entries = Vec::new();

    for (key, observations) in &grouped {
        let point = key.iter().map(|v| *v as f64).collect::<Vec<_>>();
        let (cleaned, mean, deviation) = aggregate(observations);
        let count = cleaned.len();

        let mut entry = Entry {
            point,
            mean,
            deviation,
            count,
            data: None,
        };

        if arguments.sample.enabled() {
            entry.data = Some(cleaned);
        }

        entries.push(entry);
    }

    Function {
        name: measured.name.clone(),
        tags,
        expression: "performance".to_string(),
        dimension: measured.dimensions.clone(),
        observation: Observation {
            time: Timed {
                unit: "second".to_string(),
                sample: entries,
            },
        },
    }
}

#[expect(clippy::cast_precision_loss)]
fn aggregate(observations: &[f64]) -> (Vec<f64>, f64, f64) {
    if observations.is_empty() {
        return (Vec::new(), 0.0, 0.0);
    }

    let mean = observations.iter().sum::<f64>() / observations.len() as f64;
    let variance =
        observations.iter().map(|v| (v - mean).powi(2)).sum::<f64>() / observations.len() as f64;
    let deviation = variance.sqrt();

    let cleaned = observations
        .iter()
        .copied()
        .filter(|v| (v - mean).abs() <= 2.0 * deviation)
        .collect::<Vec<_>>();

    if cleaned.is_empty() {
        return (observations.to_vec(), mean, deviation);
    }

    let mean = cleaned.iter().sum::<f64>() / cleaned.len() as f64;
    let variance = cleaned.iter().map(|v| (v - mean).powi(2)).sum::<f64>() / cleaned.len() as f64;

    (cleaned, mean, variance.sqrt())
}

#[derive(Serialize)]
struct Report {
    source: Source,
    functions: Vec<Function>,
}

#[derive(Serialize)]
struct Function {
    name: String,
    tags: Vec<String>,
    expression: String,
    dimension: Vec<String>,
    observation: Observation,
}

#[derive(Serialize)]
struct Observation {
    time: Timed,
}

#[derive(Serialize)]
struct Timed {
    unit: String,
    sample: Vec<Entry>,
}

#[derive(Serialize)]
struct Entry {
    point: Vec<f64>,
    mean: f64,
    deviation: f64,
    count: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<Vec<f64>>,
}
