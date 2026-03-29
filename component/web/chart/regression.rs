use std::fmt::Write;

use element::Element;
use scatter::Entry;
use viewport::Viewport;

pub struct Candidate {
    pub terms: Vec<Term>,
}

pub struct Term {
    pub exponent: Vec<usize>,
    pub coefficient: f64,
}

impl Candidate {
    #[must_use]
    pub fn evaluate(&self, x: f64) -> f64 {
        self.terms
            .iter()
            .map(|term| {
                let base = term
                    .exponent
                    .first()
                    .map_or(1.0, |&e| x.powi(i32::try_from(e).unwrap_or(0)));
                term.coefficient * base
            })
            .sum()
    }

    #[must_use]
    pub fn derivative(&self, x: f64) -> f64 {
        self.terms
            .iter()
            .map(|term| {
                let e = term.exponent.first().copied().unwrap_or(0);
                if e == 0 {
                    return 0.0;
                }
                let power = if e >= 2 {
                    x.powi(i32::try_from(e - 1).unwrap_or(0))
                } else {
                    1.0
                };
                term.coefficient * f64::from(u32::try_from(e).unwrap_or(0)) * power
            })
            .sum()
    }
}

#[must_use]
pub fn render(
    viewport: &Viewport,
    candidate: Option<&Candidate>,
    normalization: f64,
    entries: &[Entry],
) -> Vec<Element> {
    let mut points = String::new();
    if let Some(candidate) = candidate {
        let steps = 200;
        let span = viewport.domain.span();
        for i in 0..=steps {
            let x = viewport.domain.minimum + span * f64::from(i) / f64::from(steps);
            let y = candidate.evaluate(x / normalization);
            let projected = viewport.project(x, y);
            if !points.is_empty() {
                points.push(' ');
            }
            let _ = write!(points, "{:.1},{:.1}", projected.x, projected.y);
        }
    } else if entries.len() >= 2 {
        for entry in entries {
            let y = entry.predicted;
            let projected = viewport.project(entry.point, y);
            if !points.is_empty() {
                points.push(' ');
            }
            let _ = write!(points, "{:.1},{:.1}", projected.x, projected.y);
        }
    }
    if points.is_empty() {
        return Vec::new();
    }
    vec![Element::leaf(
        "polyline",
        vec![
            ("points".into(), points),
            ("class".into(), class::chart::winner().to_string()),
            ("clip-path".into(), "url(#plot)".into()),
        ],
    )]
}
