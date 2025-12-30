use std::collections::BTreeSet;

use component::graph::relation::Related;
use system::graph::relation::Relate;

fn new(adjacency: Vec<(String, BTreeSet<String>)>) -> Related<String> {
    Related::new(adjacency.into_iter().collect())
}

fn relate(mut related: Related<String>, label: String, relation: String) -> Related<String> {
    related.relate(&label, &relation).clone()
}
