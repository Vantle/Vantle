use component::graph::state::{Derivation, Inference, Wave};

fn resonance(inference: Inference<usize>, wave: Wave<usize>) -> Vec<Vec<Derivation<usize>>> {
    inference.resonance(&wave)
}
