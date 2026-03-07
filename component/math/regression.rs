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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Family {
    Polynomial,
    Power,
    Exponential,
}

impl Family {
    fn transform(self, sample: &Sample) -> Option<Sample> {
        match self {
            Self::Polynomial => Some(sample.clone()),
            Self::Power => {
                if sample.observation <= 0.0 || sample.point.iter().any(|&v| v <= 0.0) {
                    return None;
                }
                Some(Sample {
                    point: sample.point.iter().map(|v| v.ln()).collect::<Vec<_>>(),
                    observation: sample.observation.ln(),
                })
            }
            Self::Exponential => {
                if sample.observation <= 0.0 {
                    return None;
                }
                Some(Sample {
                    point: sample.point.clone(),
                    observation: sample.observation.ln(),
                })
            }
        }
    }

    fn inverse(self, value: f64) -> f64 {
        if matches!(self, Self::Polynomial) {
            value
        } else {
            value.exp()
        }
    }

    #[must_use]
    pub fn label(self) -> &'static str {
        match self {
            Self::Polynomial => "polynomial",
            Self::Power => "power",
            Self::Exponential => "exponential",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Candidate {
    pub family: Family,
    pub degree: usize,
    pub determination: f64,
    pub criterion: f64,
}

pub struct Selection {
    pub family: Family,
    pub degree: usize,
    pub polynomial: Polynomial,
    pub determination: f64,
    pub criterion: f64,
    pub candidates: Vec<Candidate>,
    pub residual: f64,
    pub leverage: Vec<f64>,
    dimensions: usize,
}

impl Selection {
    #[must_use]
    pub fn evaluate(&self, point: &[f64]) -> f64 {
        let transformed = if matches!(self.family, Family::Power) {
            point.iter().map(|v| v.max(1e-10).ln()).collect::<Vec<_>>()
        } else {
            point.to_vec()
        };
        let raw = self.polynomial.evaluate(&transformed);
        self.family.inverse(raw)
    }

    #[must_use]
    pub fn interval(&self, point: &[f64], confidence: f64) -> (f64, f64) {
        let transformed = if matches!(self.family, Family::Power) {
            point.iter().map(|v| v.max(1e-10).ln()).collect::<Vec<_>>()
        } else {
            point.to_vec()
        };

        let terms = monomials(self.dimensions, self.degree);
        let row = terms
            .iter()
            .map(|m| m.evaluate(&transformed))
            .collect::<Vec<_>>();

        let n = self.leverage.len();
        let k = terms.len();
        if n <= k {
            let predicted = self.family.inverse(self.polynomial.evaluate(&transformed));
            return (predicted * 0.5, predicted * 2.0);
        }

        let leverage = row.iter().map(|v| v * v).sum::<f64>() * 0.01;

        let alpha = 1.0 - confidence;
        let critical = critical(alpha, float(n - k));
        let margin = critical * self.residual * (1.0 + leverage).sqrt();

        let predicted = self.polynomial.evaluate(&transformed);
        let lower = self.family.inverse(predicted - margin);
        let upper = self.family.inverse(predicted + margin);
        (lower, upper)
    }

    #[must_use]
    pub fn interpretation(&self) -> String {
        match self.family {
            Family::Polynomial => {
                let mut parts = Vec::new();
                for term in &self.polynomial.terms {
                    let total: usize = term.monomial.exponent.iter().sum();
                    if total == 0 {
                        parts.push(format!("{:.2}", term.coefficient));
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
                        parts.push(format!("{:.2}·{vars}", term.coefficient));
                    }
                }
                format!("time ~ {}", parts.join(" + "))
            }
            Family::Power => {
                let mut parts = Vec::new();
                for term in &self.polynomial.terms {
                    let total: usize = term.monomial.exponent.iter().sum();
                    if total == 0 {
                        continue;
                    }
                    for (i, e) in term.monomial.exponent.iter().enumerate() {
                        let e = *e;
                        if e > 0 {
                            parts.push(format!("x{i}^{:.2}", term.coefficient));
                        }
                    }
                }
                if parts.is_empty() {
                    "time ~ constant".to_string()
                } else {
                    format!("time ~ {}", parts.join(" · "))
                }
            }
            Family::Exponential => {
                format!("time ~ exp(f(x)), degree {}", self.degree)
            }
        }
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

#[must_use]
pub fn fit(samples: &[Sample], terms: &[Monomial]) -> Polynomial {
    let n = samples.len();
    let k = terms.len();

    let mut vandermonde = DMatrix::zeros(n, k);
    let mut observations = DVector::zeros(n);

    for (i, sample) in samples.iter().enumerate() {
        for (j, monomial) in terms.iter().enumerate() {
            vandermonde[(i, j)] = monomial.evaluate(&sample.point);
        }
        observations[i] = sample.observation;
    }

    let svd = SVD::new(vandermonde, true, true);
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

    let families = [Family::Polynomial, Family::Power, Family::Exponential];
    let mut best: Option<(Family, usize, Polynomial, f64, f64, Vec<f64>)> = None;
    let mut candidates = Vec::new();

    for &family in &families {
        let transformed = samples
            .iter()
            .filter_map(|s| family.transform(s))
            .collect::<Vec<_>>();

        if transformed.len() < 2 {
            continue;
        }

        let effective = transformed.len();
        let cap = maximum.min(effective.saturating_sub(1));

        for degree in 1..=cap {
            let terms = monomials(dimensions, degree);
            if terms.len() >= effective {
                continue;
            }

            let polynomial = fit(&transformed, &terms);
            let k = float(terms.len());

            let mut residual_sum = 0.0;
            let mean: f64 =
                transformed.iter().map(|s| s.observation).sum::<f64>() / float(effective);
            let mut total_sum = 0.0;

            for sample in &transformed {
                let predicted = polynomial.evaluate(&sample.point);
                residual_sum += (sample.observation - predicted).powi(2);
                total_sum += (sample.observation - mean).powi(2);
            }

            let determination = if total_sum > 0.0 {
                1.0 - residual_sum / total_sum
            } else {
                1.0
            };

            let criterion = if residual_sum > 0.0 {
                float(effective) * (residual_sum / float(effective)).ln()
                    + k * (float(effective)).ln()
            } else {
                f64::NEG_INFINITY
            };

            candidates.push(Candidate {
                family,
                degree,
                determination,
                criterion,
            });

            let leverage = leverage(&transformed, &terms);

            let dominated = best
                .as_ref()
                .is_none_or(|(_, _, _, _, best_bic, _)| criterion < *best_bic);

            if dominated {
                best = Some((
                    family,
                    degree,
                    polynomial,
                    determination,
                    criterion,
                    leverage,
                ));
            }
        }
    }

    let (family, degree, polynomial, determination, criterion, leverage) = best?;
    let n_effective = leverage.len();
    let k = monomials(dimensions, degree).len();
    let residual = if n_effective > k {
        let transformed = samples
            .iter()
            .filter_map(|s| family.transform(s))
            .collect::<Vec<_>>();
        let residual_sum: f64 = transformed
            .iter()
            .map(|s| (s.observation - polynomial.evaluate(&s.point)).powi(2))
            .sum();
        (residual_sum / (float(n_effective) - float(k))).sqrt()
    } else {
        0.0
    };

    Some(Selection {
        family,
        degree,
        polynomial,
        determination,
        criterion,
        candidates,
        residual,
        leverage,
        dimensions,
    })
}

fn leverage(samples: &[Sample], terms: &[Monomial]) -> Vec<f64> {
    let n = samples.len();
    let k = terms.len();

    let mut vandermonde = DMatrix::zeros(n, k);
    for (i, sample) in samples.iter().enumerate() {
        for (j, monomial) in terms.iter().enumerate() {
            vandermonde[(i, j)] = monomial.evaluate(&sample.point);
        }
    }

    let vtv = vandermonde.transpose() * &vandermonde;
    let svd = SVD::new(vtv, true, true);

    if let Ok(inverse) = svd.pseudo_inverse(1e-10) {
        let hat = &vandermonde * inverse * vandermonde.transpose();
        (0..n).map(|i| hat[(i, i)]).collect::<Vec<_>>()
    } else {
        vec![0.0; n]
    }
}

#[expect(clippy::cast_precision_loss)]
fn float(value: usize) -> f64 {
    value as f64
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
