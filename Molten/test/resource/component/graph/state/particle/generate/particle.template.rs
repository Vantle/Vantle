use component::graph::state::Particle;
use component::graph::traits::node::Queryable;

fn disjoint(candidate: Particle<String>, basis: Particle<String>) -> Option<Particle<String>> {
    candidate.disjoint(&basis).cloned()
}

fn isomorphic(candidate: Particle<String>, basis: Particle<String>) -> Option<Particle<String>> {
    candidate.isomorphic(&basis).cloned()
}

fn joint(candidate: Particle<String>, basis: Particle<String>) -> Option<Particle<String>> {
    candidate.joint(&basis).cloned()
}

fn subset(candidate: Particle<String>, basis: Particle<String>) -> Option<Particle<String>> {
    candidate.subset(&basis).cloned()
}

fn superset(candidate: Particle<String>, basis: Particle<String>) -> Option<Particle<String>> {
    candidate.superset(&basis).cloned()
}
