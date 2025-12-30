use component::graph::state::wave::Wave;
use system::query::{Polyset, Set as QuerySet};
use system::translate::Set as TranslateSet;

fn subset(candidate: &Wave<String>, basis: &Wave<String>) -> Option<Wave<String>> {
    candidate.subset(basis).map(|_| candidate.clone())
}

fn superset(candidate: &Wave<String>, basis: &Wave<String>) -> Option<Wave<String>> {
    candidate.superset(basis).map(|_| candidate.clone())
}

fn isomorphic(candidate: &Wave<String>, basis: &Wave<String>) -> Option<Wave<String>> {
    candidate.isomorphic(basis).map(|_| candidate.clone())
}

fn joint(candidate: &Wave<String>, basis: &Wave<String>) -> Option<Wave<String>> {
    candidate.joint(basis).map(|_| candidate.clone())
}

fn disjoint(candidate: &Wave<String>, basis: &Wave<String>) -> Option<Wave<String>> {
    candidate.disjoint(basis).map(|_| candidate.clone())
}

fn join(candidate: &Wave<String>, basis: &Wave<String>) -> Option<Wave<String>> {
    candidate.join(basis)
}

fn intersect(candidate: &Wave<String>, basis: &Wave<String>) -> Option<Wave<String>> {
    candidate.intersect(basis)
}

fn diverge(candidate: &Wave<String>, basis: &Wave<String>) -> Option<Wave<String>> {
    candidate.diverge(basis)
}

fn diverges(candidate: &Wave<String>, basis: &Wave<String>) -> Vec<Wave<String>> {
    candidate.diverges(basis).into_iter().collect()
}
