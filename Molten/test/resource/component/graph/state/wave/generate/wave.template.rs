use component::graph::state::Wave;
use component::graph::traits::node::{Polytranslatable, Queryable, Translatable};

fn subset(candidate: Wave<String>, basis: Wave<String>) -> Option<Wave<String>> {
    candidate.subset(&basis).cloned()
}

fn superset(candidate: Wave<String>, basis: Wave<String>) -> Option<Wave<String>> {
    candidate.superset(&basis).cloned()
}

fn isomorphic(candidate: Wave<String>, basis: Wave<String>) -> Option<Wave<String>> {
    candidate.isomorphic(&basis).cloned()
}

fn joint(candidate: Wave<String>, basis: Wave<String>) -> Option<Wave<String>> {
    candidate.joint(&basis).cloned()
}

fn disjoint(candidate: Wave<String>, basis: Wave<String>) -> Option<Wave<String>> {
    candidate.disjoint(&basis).cloned()
}

fn join(candidate: Wave<String>, basis: Wave<String>) -> Option<Wave<String>> {
    candidate.join(&basis)
}

fn intersect(candidate: Wave<String>, basis: Wave<String>) -> Option<Wave<String>> {
    candidate.intersect(&basis)
}

fn diverge(candidate: Wave<String>, basis: Wave<String>) -> Option<Wave<String>> {
    candidate.diverge(&basis)
}

fn diverges(candidate: Wave<String>, basis: Wave<String>) -> Vec<Wave<String>> {
    candidate.diverges(&basis).into_iter().collect()
}
