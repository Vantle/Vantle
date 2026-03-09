use component::graph::state::particle::Particle;
use component::graph::state::wave::Wave;
use system::scale::Scaled;

fn particle(source: Particle<String>, basis: String) -> usize {
    source.scale(&basis)
}

fn wave(source: Wave<String>, basis: Particle<String>) -> usize {
    source.scale(&basis)
}
