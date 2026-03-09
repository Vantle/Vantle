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
pub struct Assertion {
    pub terms: Vec<(Vec<usize>, f64)>,
    pub confidence: f64,
}

pub struct Timing {
    pub point: Vec<f64>,
    pub observation: f64,
}

pub struct Measured {
    pub name: String,
    pub dimensions: Vec<String>,
    pub bounds: Vec<Assertion>,
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

            if let Some(classification) = function.observation.time.classification {
                for bound in &function.observation.time.candidate[classification].bound {
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
fn analyze(measured: &Measured, arguments: &Arguments) -> Function {
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

        let mut entry = Entry {
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
        return Function {
            name: measured.name.clone(),
            expression: "performance".to_string(),
            dimension: measured.dimensions.clone(),
            observation: Observation {
                time: Timed {
                    unit: "second".to_string(),
                    classification: None,
                    candidate: Vec::new(),
                    sample: entries,
                },
            },
        };
    }

    let dimensions = measured.dimensions.len().max(1);
    let selection = regression::select(&samples, dimensions, 5);

    let mut classification = None;
    let mut candidates = Vec::new();

    if let Some(ref selection) = selection {
        for sample in &mut entries {
            if arguments.interval.enabled() {
                let (lower, upper) = selection.interval(&sample.point, 0.95);
                sample.interval = [lower, upper];
            }
        }

        if arguments.model.enabled() {
            classification = Some(selection.classification);

            for c in &selection.candidates {
                let terms = c
                    .polynomial
                    .terms
                    .iter()
                    .map(|term| Term {
                        exponent: term.monomial.exponent.clone(),
                        coefficient: term.coefficient,
                    })
                    .collect::<Vec<_>>();

                let interpretation = selection.interpret(&c.polynomial);

                let bound = if arguments.bound.enabled() {
                    check(&measured.bounds, &samples, &c.polynomial)
                } else {
                    Vec::new()
                };

                candidates.push(Candidate {
                    degree: c.degree,
                    term: terms,
                    interpretation: Interpretation {
                        expression: interpretation.structure,
                        scale: interpretation.scale,
                    },
                    determination: c.determination,
                    criterion: c.criterion,
                    bound,
                });
            }
        }
    }

    Function {
        name: measured.name.clone(),
        expression: "performance".to_string(),
        dimension: measured.dimensions.clone(),
        observation: Observation {
            time: Timed {
                unit: "second".to_string(),
                classification,
                candidate: candidates,
                sample: entries,
            },
        },
    }
}

fn check(
    bounds: &[Assertion],
    samples: &[Sample],
    polynomial: &regression::Polynomial,
) -> Vec<Constraint> {
    let contributions = polynomial
        .terms
        .iter()
        .map(|term| {
            let value = samples
                .iter()
                .map(|s| (term.coefficient * term.monomial.evaluate(&s.point)).abs())
                .fold(0.0_f64, f64::max);
            (term.monomial.exponent.clone(), value)
        })
        .collect::<HashMap<_, _>>();

    let mut constraints = Vec::new();
    for bound in bounds {
        let mut sorted = bound.terms.clone();
        sorted.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

        let mut status = "pass";
        let mut violated = None;
        for pair in sorted.windows(2) {
            let higher = contributions.get(&pair[0].0).copied().unwrap_or(0.0);
            let lower = contributions.get(&pair[1].0).copied().unwrap_or(0.0);
            if higher < lower * bound.confidence {
                status = "fail";
                violated = Some(format!(
                    "{:?} < {:?} * {}",
                    pair[0].0, pair[1].0, bound.confidence
                ));
                break;
            }
        }

        let structure: HashMap<String, f64> = bound
            .terms
            .iter()
            .map(|(exp, weight)| (format!("{exp:?}"), *weight))
            .collect();

        let assertion = serde_json::json!({
            "structure": structure,
            "confidence": bound.confidence,
        });

        constraints.push(Constraint {
            assertion,
            violated,
            status: status.to_string(),
        });
    }
    constraints
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
    #[serde(skip_serializing_if = "Option::is_none")]
    classification: Option<usize>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    candidate: Vec<Candidate>,
    sample: Vec<Entry>,
}

#[derive(Serialize)]
struct Candidate {
    degree: usize,
    term: Vec<Term>,
    interpretation: Interpretation,
    determination: f64,
    criterion: f64,
    bound: Vec<Constraint>,
}

#[derive(Serialize)]
struct Interpretation {
    expression: String,
    scale: f64,
}

#[derive(Serialize)]
struct Term {
    exponent: Vec<usize>,
    coefficient: f64,
}

#[derive(Serialize)]
struct Entry {
    point: Vec<f64>,
    mean: f64,
    deviation: f64,
    interval: [f64; 2],
    count: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<Vec<f64>>,
}

#[derive(Serialize)]
struct Constraint {
    assertion: Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    violated: Option<String>,
    status: String,
}
