use component::graph::matrix::Related;
use component::graph::state::{Inference, Wave};
use logging::info;

pub fn solidify(
    input: &Wave<usize>,
    output: &Wave<usize>,
    context: &Related<Wave<usize>>,
    limit: Option<usize>,
) -> bool {
    info!("Solidifying: checking if output is reachable from input");

    let mut inference = Inference::new();
    if let Err(error) = lava::propagate(&mut inference, input, context, limit) {
        info!("Solidification failed: {}", error);
        return false;
    }

    let reachable = output
        .particles()
        .all(|particle| inference.contains(particle));

    info!(
        "Solidification {}: {} of {} particles reachable",
        if reachable { "succeeded" } else { "failed" },
        output.particles().filter(|p| inference.contains(p)).count(),
        output.count()
    );

    reachable
}
