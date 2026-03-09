use nalgebra::{DMatrix, DVector, SVD};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Monomial {
    pub exponent: Vec<usize>,
}

impl Monomial {
    #[must_use]
    pub fn evaluate(&self, point: &[f64]) -> f64 {
        self.exponent
            .iter()
            .zip(point.iter())
            .map(|(power, base)| {
                let exponent = i32::try_from(*power).unwrap_or(i32::MAX);
                base.powi(exponent)
            })
            .product()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Term {
    pub monomial: Monomial,
    pub coefficient: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Polynomial {
    pub terms: Vec<Term>,
}

impl Polynomial {
    #[must_use]
    pub fn evaluate(&self, point: &[f64]) -> f64 {
        self.terms
            .iter()
            .map(|term| term.coefficient * term.monomial.evaluate(point))
            .sum()
    }
}

#[derive(Debug, Clone)]
pub struct Sample {
    pub point: Vec<f64>,
    pub observation: f64,
}

pub struct Interpretation {
    pub structure: String,
    pub scale: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Candidate {
    pub degree: usize,
    pub polynomial: Polynomial,
    pub determination: f64,
    pub criterion: f64,
}

pub struct Selection {
    pub classification: usize,
    pub candidates: Vec<Candidate>,
    pub residual: f64,
    dimensions: usize,
    samples: Vec<Sample>,
    normalization: Vec<f64>,
    gram: DMatrix<f64>,
}

impl Selection {
    #[must_use]
    pub fn winner(&self) -> &Candidate {
        &self.candidates[self.classification]
    }
}

impl Selection {
    #[must_use]
    pub fn evaluate(&self, point: &[f64]) -> f64 {
        let normalized = self.normalize(point);
        self.winner().polynomial.evaluate(&normalized)
    }

    #[must_use]
    pub fn interval(&self, point: &[f64], confidence: f64) -> (f64, f64) {
        let normalized = self.normalize(point);
        let winner = self.winner();
        let terms = monomials(self.dimensions, winner.degree);

        let n = self.samples.len();
        let k = terms.len();
        if n <= k {
            let predicted = winner.polynomial.evaluate(&normalized);
            return (predicted * 0.5, predicted * 2.0);
        }

        let row: Vec<f64> = terms.iter().map(|m| m.evaluate(&normalized)).collect();
        let x = DVector::from_vec(row);
        let leverage = (&x.transpose() * &self.gram * &x)[(0, 0)];

        let alpha = 1.0 - confidence;
        let critical = critical(alpha, float(n - k));
        let margin = critical * self.residual * (1.0 + leverage).sqrt();

        let predicted = winner.polynomial.evaluate(&normalized);
        (predicted - margin, predicted + margin)
    }

    #[must_use]
    pub fn interpretation(&self) -> Interpretation {
        self.interpret(&self.winner().polynomial)
    }

    #[must_use]
    pub fn interpret(&self, polynomial: &Polynomial) -> Interpretation {
        let contributions = self.contributions(polynomial);
        let peak = contributions.iter().copied().fold(0.0_f64, f64::max);
        let threshold = peak * 0.01;

        let dominant = polynomial
            .terms
            .iter()
            .zip(contributions.iter())
            .filter(|(_, c)| **c >= threshold)
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
            .map(|(term, _)| term.coefficient.abs());

        let scale = match dominant {
            Some(s) if s > 0.0 => s,
            _ => {
                return Interpretation {
                    structure: "0".to_string(),
                    scale: 0.0,
                };
            }
        };

        let mut structure = String::new();
        for (term, contribution) in polynomial.terms.iter().zip(contributions.iter()) {
            if *contribution < threshold {
                continue;
            }
            let normalized = term.coefficient / scale;
            let magnitude = normalized.abs();
            let negative = normalized < 0.0;

            if structure.is_empty() {
                if negative {
                    structure.push('-');
                }
            } else if negative {
                structure.push_str(" - ");
            } else {
                structure.push_str(" + ");
            }

            let total: usize = term.monomial.exponent.iter().sum();
            if total == 0 {
                structure.push_str(&notation(magnitude));
            } else {
                let vars = term
                    .monomial
                    .exponent
                    .iter()
                    .copied()
                    .enumerate()
                    .filter(|(_, e)| *e > 0)
                    .map(|(i, e)| {
                        if e == 1 {
                            format!("x{i}")
                        } else {
                            format!("x{i}^{e}")
                        }
                    })
                    .collect::<Vec<_>>()
                    .join("·");
                if (magnitude - 1.0).abs() < 1e-6 {
                    structure.push_str(&vars);
                } else {
                    use std::fmt::Write;
                    let _ = write!(structure, "{}·{vars}", notation(magnitude));
                }
            }
        }

        if structure.is_empty() {
            structure.push('0');
        }

        Interpretation { structure, scale }
    }

    #[must_use]
    pub fn contributions(&self, polynomial: &Polynomial) -> Vec<f64> {
        polynomial
            .terms
            .iter()
            .map(|term| {
                self.samples
                    .iter()
                    .map(|sample| (term.coefficient * term.monomial.evaluate(&sample.point)).abs())
                    .fold(0.0_f64, f64::max)
            })
            .collect::<Vec<_>>()
    }

    fn normalize(&self, point: &[f64]) -> Vec<f64> {
        point
            .iter()
            .zip(&self.normalization)
            .map(|(v, n)| v / n)
            .collect::<Vec<_>>()
    }
}

#[must_use]
pub fn monomials(dimensions: usize, degree: usize) -> Vec<Monomial> {
    let mut result = Vec::new();
    let mut exponent = vec![0usize; dimensions];
    enumerate(&mut result, &mut exponent, 0, degree);
    result
}

fn enumerate(
    result: &mut Vec<Monomial>,
    exponent: &mut Vec<usize>,
    index: usize,
    remaining: usize,
) {
    if index == exponent.len() {
        result.push(Monomial {
            exponent: exponent.clone(),
        });
        return;
    }
    for power in 0..=remaining {
        exponent[index] = power;
        enumerate(result, exponent, index + 1, remaining - power);
    }
    exponent[index] = 0;
}

fn vandermonde(samples: &[Sample], terms: &[Monomial]) -> DMatrix<f64> {
    let n = samples.len();
    let k = terms.len();
    let mut matrix = DMatrix::zeros(n, k);
    for (i, sample) in samples.iter().enumerate() {
        for (j, monomial) in terms.iter().enumerate() {
            matrix[(i, j)] = monomial.evaluate(&sample.point);
        }
    }
    matrix
}

#[must_use]
pub fn fit(samples: &[Sample], terms: &[Monomial]) -> Polynomial {
    let k = terms.len();
    let matrix = vandermonde(samples, terms);
    let mut observations = DVector::zeros(samples.len());
    for (i, sample) in samples.iter().enumerate() {
        observations[i] = sample.observation;
    }

    let svd = SVD::new(matrix, true, true);
    let coefficients = svd
        .solve(&observations, 1e-10)
        .unwrap_or_else(|_| DVector::zeros(k));

    Polynomial {
        terms: terms
            .iter()
            .zip(coefficients.iter())
            .map(|(monomial, &coefficient)| Term {
                monomial: monomial.clone(),
                coefficient,
            })
            .collect::<Vec<_>>(),
    }
}

#[must_use]
pub fn select(samples: &[Sample], dimensions: usize, maximum: usize) -> Option<Selection> {
    let n = samples.len();
    if n < 2 {
        return None;
    }

    let normalization: Vec<f64> = (0..dimensions)
        .map(|d| {
            samples
                .iter()
                .map(|s| s.point.get(d).copied().unwrap_or(0.0).abs())
                .fold(1.0_f64, f64::max)
        })
        .collect();

    let normalized: Vec<Sample> = samples
        .iter()
        .map(|s| Sample {
            point: s
                .point
                .iter()
                .zip(&normalization)
                .map(|(v, scale)| v / scale)
                .collect(),
            observation: s.observation,
        })
        .collect();

    let cap = maximum.min(n.saturating_sub(1));
    let mut best: Option<usize> = None;
    let mut candidates = Vec::new();

    for degree in 1..=cap {
        let terms = monomials(dimensions, degree);
        if terms.len() >= n {
            continue;
        }

        let polynomial = fit(&normalized, &terms);
        let k = float(terms.len());

        let mean: f64 = normalized.iter().map(|s| s.observation).sum::<f64>() / float(n);
        let mut residual = 0.0;
        let mut total = 0.0;

        for sample in &normalized {
            let predicted = polynomial.evaluate(&sample.point);
            residual += (sample.observation - predicted).powi(2);
            total += (sample.observation - mean).powi(2);
        }

        let determination = if total > 0.0 {
            1.0 - residual / total
        } else {
            1.0
        };

        let criterion = if residual > 0.0 {
            float(n) * (residual / float(n)).ln() + k * float(n).ln()
        } else {
            f64::NEG_INFINITY
        };

        let index = candidates.len();
        candidates.push(Candidate {
            degree,
            polynomial,
            determination,
            criterion,
        });

        let dominated = best
            .as_ref()
            .is_none_or(|&i| criterion < candidates[i].criterion);

        if dominated {
            best = Some(index);
        }
    }

    let classification = best?;
    let winner = &candidates[classification];
    let terms = monomials(dimensions, winner.degree);
    let k = terms.len();

    let residual = if n > k {
        let squared: f64 = normalized
            .iter()
            .map(|s| (s.observation - winner.polynomial.evaluate(&s.point)).powi(2))
            .sum();
        (squared / (float(n) - float(k))).sqrt()
    } else {
        0.0
    };

    let matrix = vandermonde(&normalized, &terms);
    let gram = pseudoinverse(&matrix);

    Some(Selection {
        classification,
        candidates,
        residual,
        dimensions,
        samples: normalized,
        normalization,
        gram,
    })
}

fn pseudoinverse(matrix: &DMatrix<f64>) -> DMatrix<f64> {
    let k = matrix.ncols();
    let vtv = matrix.transpose() * matrix;
    let svd = SVD::new(vtv, true, true);
    svd.pseudo_inverse(1e-10)
        .unwrap_or_else(|_| DMatrix::identity(k, k))
}

#[expect(clippy::cast_precision_loss)]
fn float(value: usize) -> f64 {
    value as f64
}

fn notation(value: f64) -> String {
    if value == 0.0 {
        return "0".to_string();
    }
    let magnitude = value.abs().log10().floor();
    if (-2.0..=3.0).contains(&magnitude) {
        match (2.0 - magnitude).max(0.0) {
            d if d < 0.5 => format!("{value:.0}"),
            d if d < 1.5 => format!("{value:.1}"),
            d if d < 2.5 => format!("{value:.2}"),
            d if d < 3.5 => format!("{value:.3}"),
            _ => format!("{value:.4}"),
        }
    } else {
        let power = 10_f64.powf(magnitude);
        let mantissa = value / power;
        format!("{mantissa:.2}e{magnitude}")
    }
}

fn critical(alpha: f64, degrees: f64) -> f64 {
    if degrees < 1.0 {
        return 12.706;
    }
    let z: f64 = if alpha <= 0.01 {
        2.576
    } else if alpha <= 0.05 {
        1.960
    } else {
        1.645
    };
    z + (z.powi(3) + z) / (4.0 * degrees)
}
