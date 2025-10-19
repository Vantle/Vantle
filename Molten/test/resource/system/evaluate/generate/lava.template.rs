use component::graph::matrix::Related;
use component::graph::state::{Inference, Wave};
use system::evaluation::evaluator::lava;

fn propegate(
    signal: Wave<usize>,
    context: Related<Wave<usize>>,
    limit: Option<usize>,
) -> Inference<usize> {
    let mut inference = Inference::new();
    utility::unwrap(lava::propagate(&mut inference, &signal, &context, limit));
    inference
}
