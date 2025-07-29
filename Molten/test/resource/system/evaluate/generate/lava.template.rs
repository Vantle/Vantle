use component::graph::matrix::Related;
use component::graph::state::Wave;

fn reduce(signal: Wave<usize>, context: Related<Wave<usize>>) -> Wave<usize> {
    system::evaluate::lava::reduce(&signal, &context)
}
