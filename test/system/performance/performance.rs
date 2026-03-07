pub use error;

use std::collections::HashMap;
use std::io::Write;
use std::path::PathBuf;

use clap::Parser;
use regression::Sample;
use serde::Serialize;
use serde_json::Value;

use error::{Error, Violation};

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
    #[arg(long, default_value = "on")]
    pub model: Toggle,
    #[arg(long, default_value = "on")]
    pub bound: Toggle,
    #[arg(long, default_value = "on")]
    pub interval: Toggle,
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

#[derive(Clone)]
pub enum BoundAssertion {
    At {
        point: HashMap<String, usize>,
        within: std::time::Duration,
    },
    Determination(f64),
}

pub struct Timing {
    pub point: Vec<f64>,
    pub observation: f64,
}

pub struct Measured {
    pub name: String,
    pub dimensions: Vec<String>,
    pub bounds: Vec<BoundAssertion>,
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
        let mut violations = Vec::new();

        for measured in self.functions {
            let function = analyze(&measured, &self.arguments);

            for bound in &function.observation.time.bound {
                if bound.status == "fail" {
                    violations.push(Violation::new(
                        measured.name.clone(),
                        format!(
                            "bound violated: {}",
                            serde_json::to_string(&bound.assertion).unwrap_or_default()
                        ),
                    ));
                }
            }
            report.functions.push(function);
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

        if !violations.is_empty() {
            return Err(Error::collection(violations).into());
        }
        Ok(())
    }
}

#[expect(clippy::cast_precision_loss, clippy::cast_possible_truncation)]
fn analyze(measured: &Measured, arguments: &Arguments) -> FunctionReport {
    let mut grouped: HashMap<Vec<i64>, Vec<f64>> = HashMap::new();

    for timing in &measured.timings {
        let key = timing.point.iter().map(|v| *v as i64).collect::<Vec<_>>();
        grouped.entry(key).or_default().push(timing.observation);
    }

    let mut samples = Vec::new();
    let mut entries = Vec::new();

    for (key, observations) in &grouped {
        let point = key.iter().map(|v| *v as f64).collect::<Vec<_>>();
        let (cleaned, mean, deviation) = aggregate(observations);
        let count = cleaned.len();

        samples.push(Sample {
            point: point.clone(),
            observation: mean,
        });

        let mut entry = SampleReport {
            point: point.clone(),
            mean,
            deviation,
            interval: [0.0, 0.0],
            count,
            data: None,
        };

        if arguments.sample.enabled() {
            entry.data = Some(cleaned);
        }

        entries.push(entry);
    }

    if samples.len() < 2 {
        return FunctionReport {
            function: measured.name.clone(),
            expression: "performance".to_string(),
            dimension: measured.dimensions.clone(),
            observation: ObservationReport {
                time: TimeReport {
                    unit: "second".to_string(),
                    model: None,
                    sample: entries,
                    bound: Vec::new(),
                },
            },
        };
    }

    let dimensions = measured.dimensions.len().max(1);
    let selection = regression::select(&samples, dimensions, 5);

    let mut model = None;
    let mut assertions = Vec::new();

    if let Some(ref selection) = selection {
        for sample in &mut entries {
            if arguments.interval.enabled() {
                let (lower, upper) = selection.interval(&sample.point, 0.95);
                sample.interval = [lower, upper];
            }
        }

        if arguments.model.enabled() {
            let terms = selection
                .polynomial
                .terms
                .iter()
                .map(|term| TermReport {
                    exponent: term.monomial.exponent.clone(),
                    coefficient: term.coefficient,
                })
                .collect::<Vec<_>>();

            let candidate = selection
                .candidates
                .iter()
                .map(|c| CandidateReport {
                    family: c.family.label().to_string(),
                    degree: c.degree,
                    criterion: c.criterion,
                })
                .collect::<Vec<_>>();

            model = Some(ModelReport {
                family: selection.family.label().to_string(),
                degree: selection.degree,
                term: terms,
                interpretation: selection.interpretation(),
                determination: selection.determination,
                criterion: selection.criterion,
                candidate,
            });
        }

        if arguments.bound.enabled() {
            for bound in &measured.bounds {
                match bound {
                    BoundAssertion::At { point, within } => {
                        let evaluation = measured
                            .dimensions
                            .iter()
                            .map(|d| *point.get(d).unwrap_or(&0) as f64)
                            .collect::<Vec<_>>();

                        let predicted = selection.evaluate(&evaluation);
                        let (lower, upper) = selection.interval(&evaluation, 0.95);
                        let within_seconds = within.as_secs_f64();

                        let status = if predicted <= within_seconds {
                            "pass"
                        } else {
                            "fail"
                        };

                        let assertion = serde_json::json!({
                            "at": point,
                            "within": performance::format(*within),
                        });

                        assertions.push(BoundReport {
                            assertion,
                            predicted: Some(performance::format(
                                std::time::Duration::from_secs_f64(predicted.max(0.0)),
                            )),
                            interval: Some([
                                performance::format(std::time::Duration::from_secs_f64(
                                    lower.max(0.0),
                                )),
                                performance::format(std::time::Duration::from_secs_f64(
                                    upper.max(0.0),
                                )),
                            ]),
                            actual: None,
                            status: status.to_string(),
                        });
                    }
                    BoundAssertion::Determination(threshold) => {
                        let status = if selection.determination >= *threshold {
                            "pass"
                        } else {
                            "fail"
                        };

                        assertions.push(BoundReport {
                            assertion: serde_json::json!({ "determination": threshold }),
                            predicted: None,
                            interval: None,
                            actual: Some(selection.determination),
                            status: status.to_string(),
                        });
                    }
                }
            }
        }
    }

    FunctionReport {
        function: measured.name.clone(),
        expression: "performance".to_string(),
        dimension: measured.dimensions.clone(),
        observation: ObservationReport {
            time: TimeReport {
                unit: "second".to_string(),
                model,
                sample: entries,
                bound: assertions,
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
    functions: Vec<FunctionReport>,
}

#[derive(Serialize)]
struct FunctionReport {
    function: String,
    expression: String,
    dimension: Vec<String>,
    observation: ObservationReport,
}

#[derive(Serialize)]
struct ObservationReport {
    time: TimeReport,
}

#[derive(Serialize)]
struct TimeReport {
    unit: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    model: Option<ModelReport>,
    sample: Vec<SampleReport>,
    bound: Vec<BoundReport>,
}

#[derive(Serialize)]
struct ModelReport {
    family: String,
    degree: usize,
    term: Vec<TermReport>,
    interpretation: String,
    determination: f64,
    criterion: f64,
    candidate: Vec<CandidateReport>,
}

#[derive(Serialize)]
struct TermReport {
    exponent: Vec<usize>,
    coefficient: f64,
}

#[derive(Serialize)]
struct CandidateReport {
    family: String,
    degree: usize,
    criterion: f64,
}

#[derive(Serialize)]
struct SampleReport {
    point: Vec<f64>,
    mean: f64,
    deviation: f64,
    interval: [f64; 2],
    count: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<Vec<f64>>,
}

#[derive(Serialize)]
struct BoundReport {
    assertion: Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    predicted: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    interval: Option<[String; 2]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    actual: Option<f64>,
    status: String,
}
