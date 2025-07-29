use component::graph::matrix::Related;
use component::graph::state::Wave;
use component::graph::traits::node::{Polytranslatable, Translatable};

pub fn deduce(_from: &Wave<usize>, _context: &Related<Wave<usize>>) -> Wave<usize> {
    todo!()
}

pub fn reduce(signal: &Wave<usize>, context: &Related<Wave<usize>>) -> Wave<usize> {
    let mut reduction = Wave::default();
    for (wave, modulations) in context {
        let divergences = signal.diverges(wave);
        for divergence in &divergences {
            for modulation in modulations {
                let advance = divergence.join(modulation).unwrap_or(divergence.clone());
                reduction = reduction.join(&advance).unwrap_or(reduction.clone());
            }
        }
    }
    reduction
}
