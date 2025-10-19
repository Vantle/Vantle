use component::graph::state::{Particle, Wave};
use system::evaluation::evaluator::{Hypergraph, Label};

fn focus(graph: &mut Hypergraph<usize>, particle: Particle<usize>) -> Label {
    graph.focus(particle)
}

fn diffuse(graph: &mut Hypergraph<usize>, signal: Wave<usize>) -> Vec<Label> {
    graph.diffuse(signal).collect()
}

fn united(graph: &Hypergraph<usize>) -> Vec<Vec<Label>> {
    graph.united().map(|iter| iter.collect()).collect()
}

fn node(graph: &Hypergraph<usize>, label: Label) -> Label {
    utility::unwrap(graph.node(label)).label
}

fn edge(graph: &Hypergraph<usize>, label: Label) -> Label {
    utility::unwrap(graph.edge(label)).label
}

fn independent(graph: &Hypergraph<usize>, rank: usize) -> Vec<Vec<Label>> {
    graph.independent(rank).collect()
}
