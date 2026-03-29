use analysis::{Fit, Observation, Predicted, Term};

pub struct Result {
    pub fit: Option<Fit>,
    pub samples: Vec<Predicted>,
}

#[must_use]
pub fn perform(observations: &[Observation], count: usize) -> Result {
    if observations.len() < 2 {
        return Result {
            fit: None,
            samples: observations
                .iter()
                .map(|o| Predicted {
                    point: o.point.clone(),
                    mean: o.mean,
                    deviation: o.deviation,
                    count: o.count,
                    predicted: o.mean,
                    interval: [0.0, 0.0],
                })
                .collect::<Vec<_>>(),
        };
    }

    let mut normalization = 1.0_f64;
    let input = observations
        .iter()
        .map(|o| {
            normalization = normalization.max(o.point.first().copied().unwrap_or(0.0).abs());
            regression::Sample {
                point: o.point.clone(),
                observation: o.mean,
            }
        })
        .collect::<Vec<_>>();

    let selection = regression::select(&input, count.max(1), 5);

    let fit = selection.as_ref().map(|selection| {
        let winner = selection.winner();
        Fit {
            terms: winner
                .polynomial
                .terms
                .iter()
                .map(|term| Term {
                    exponent: term.monomial.exponent.clone(),
                    coefficient: term.coefficient,
                })
                .collect::<Vec<_>>(),
            normalization,
        }
    });

    let samples = observations
        .iter()
        .map(|o| {
            let predicted = selection.as_ref().map_or(o.mean, |s| s.evaluate(&o.point));
            let interval = selection.as_ref().map_or([0.0, 0.0], |s| {
                let (lower, upper) = s.interval(&o.point, 0.95);
                [lower, upper]
            });
            Predicted {
                point: o.point.clone(),
                mean: o.mean,
                deviation: o.deviation,
                count: o.count,
                predicted,
                interval,
            }
        })
        .collect::<Vec<_>>();

    Result { fit, samples }
}
