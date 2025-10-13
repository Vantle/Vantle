use component::graph::matrix::Related;
use component::graph::state::{Inference, Wave};
use system::evaluate::lava;

fn propegate(
    signal: Wave<usize>,
    context: Related<Wave<usize>>,
    limit: Option<usize>,
) -> Inference<usize> {
    let mut inference = Inference::new();
    lava::propagate(&mut inference, &signal, &context, limit)
        .expect("Inference iteration limit exceeded");
    inference
}
