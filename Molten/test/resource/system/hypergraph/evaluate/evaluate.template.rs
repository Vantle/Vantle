use std::collections::BTreeSet;

use component::graph::relation::Edge as Relation;
use component::graph::relation::Related;
use component::graph::state::{particle::Particle, wave::Wave};
use component::hypergraph::{Hypergraph, Inference, Label, Translation};
use system::hypergraph::evaluate::Evaluate;

fn focus(graph: &mut Hypergraph<usize>, particle: Particle<usize>) -> Label {
    graph.focus(particle)
}

fn diffuse(graph: &mut Hypergraph<usize>, signal: Wave<usize>) -> Vec<Label> {
    graph.diffuse(signal).collect()
}

fn united(graph: &Hypergraph<usize>) -> Vec<Vec<Label>> {
    graph.united().map(Iterator::collect).collect()
}

fn node(graph: &Hypergraph<usize>, label: Label) -> Label {
    utility::unwrap(graph.node(label)).label
}

fn edge(graph: &Hypergraph<usize>, label: Label) -> Label {
    utility::unwrap(graph.edge(label)).label
}

fn independent(graph: &Hypergraph<usize>, rank: usize) -> Vec<BTreeSet<Label>> {
    graph.independent(rank).collect()
}

fn bipartitions(source: &Wave<usize>, sink: &Wave<usize>) -> Vec<Wave<usize>> {
    system::hypergraph::evaluate::bipartitions(source, sink).collect()
}

fn absorb(
    graph: &mut Hypergraph<usize>,
    source: BTreeSet<Label>,
    relation: Relation<Wave<usize>>,
) -> Vec<Label> {
    utility::unwrap(graph.absorb(source, relation)).collect()
}

fn translate(
    graph: &mut Hypergraph<usize>,
    source: BTreeSet<Label>,
    destinations: BTreeSet<Label>,
    relation: Relation<Wave<usize>>,
) -> Translation {
    utility::unwrap(graph.translate(source, destinations, relation))
}

fn infer(graph: &mut Hypergraph<usize>, refractions: Related<Wave<usize>>) -> Inference {
    utility::unwrap(graph.infer(refractions))
}

fn fixed(graph: &mut Hypergraph<usize>, refractions: Related<Wave<usize>>) -> Inference {
    utility::unwrap(graph.fixed(refractions))
}

fn isomorphics(graph: &Hypergraph<usize>, particle: &Particle<usize>) -> Vec<Label> {
    graph.isomorphics(particle).collect()
}
