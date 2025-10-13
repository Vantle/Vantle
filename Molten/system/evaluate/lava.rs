use component::graph::matrix::Related;
use component::graph::state::{Inference, Particle, Wave};
use component::graph::traits::node::Translatable;
use logging::info;
use miette::Diagnostic;
use thiserror::Error;

#[derive(Error, Debug, Diagnostic)]
pub enum Error {
    #[error("Iteration limit exceeded: {iterations} iterations reached (limit: {limit})")]
    #[diagnostic(
        code(inference::limit),
        help("The inference graph is still evolving after {iterations} iterations. Graph has {particles} particles. Consider increasing the limit or checking for unbounded growth patterns.")
    )]
    Limit {
        iterations: usize,
        limit: usize,
        particles: usize,
    },
}

impl Error {
    pub fn code(&self) -> i32 {
        match self {
            Self::Limit { .. } => 70,
        }
    }

    pub fn limit(iterations: usize, limit: usize, particles: usize) -> Self {
        Self::Limit {
            iterations,
            limit,
            particles,
        }
    }
}

pub fn infer<'a>(
    inference: &'a mut Inference<usize>,
    context: &Related<Wave<usize>>,
) -> Option<&'a Inference<usize>> {
    let mut changed = false;
    for (source, destinations) in context {
        info!(
            "Checking rule: input wave with {} particles",
            source.count()
        );

        let quantums: Vec<_> = source.particles().cloned().collect();
        let matchings = inference.matching(source);

        info!("Found {} matchings", matchings.len());

        for matched in matchings {
            let residue = matched
                .iter()
                .zip(&quantums)
                .filter_map(|(label, quantum)| {
                    let particle = inference.node(label)?;
                    particle.diverge(quantum)
                })
                .reduce(|construction, divergence| {
                    construction.join(&divergence).unwrap_or(divergence)
                })
                .unwrap_or_else(Particle::empty);

            let evolutions: Vec<_> = destinations
                .iter()
                .flat_map(|wave| wave.particles())
                .map(|reflection| {
                    reflection
                        .join(&residue)
                        .unwrap_or_else(|| reflection.clone())
                })
                .collect();

            info!(
                "Matched {} particles, {} destinations",
                matched.len(),
                evolutions.len()
            );

            if !evolutions.is_empty() && inference.infer(&matched, &evolutions) {
                changed = true;
                info!("Applied, graph has {} particles", inference.len());
            }
        }
    }

    changed.then_some(inference)
}

pub fn propagate(
    inference: &mut Inference<usize>,
    signal: &Wave<usize>,
    context: &Related<Wave<usize>>,
    bound: Option<usize>,
) -> Result<(), Error> {
    let bound = bound.unwrap_or(usize::MAX);

    signal.particles().for_each(|particle| {
        inference.insert(particle.clone());
    });

    for iteration in 1..=bound {
        if infer(inference, context).is_none() {
            return Ok(());
        }
        info!(
            "Iteration {}: graph has {} particles",
            iteration,
            inference.len()
        );
    }

    Err(Error::limit(bound, bound, inference.len()))
}
