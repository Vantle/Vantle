use component::graph::state::particle::Particle;
use system::query::Set;

fn disjoint(candidate: Particle<String>, basis: Particle<String>) -> Option<Particle<String>> {
    candidate.disjoint(&basis).map(|_| candidate.clone())
}

fn isomorphic(candidate: Particle<String>, basis: Particle<String>) -> Option<Particle<String>> {
    candidate.isomorphic(&basis).map(|_| candidate.clone())
}

fn joint(candidate: Particle<String>, basis: Particle<String>) -> Option<Particle<String>> {
    candidate.joint(&basis).map(|_| candidate.clone())
}

fn subset(candidate: Particle<String>, basis: Particle<String>) -> Option<Particle<String>> {
    candidate.subset(&basis).map(|_| candidate.clone())
}

fn superset(candidate: Particle<String>, basis: Particle<String>) -> Option<Particle<String>> {
    candidate.superset(&basis).map(|_| candidate.clone())
}
