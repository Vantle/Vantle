use component::graph::relation::Related;
use system::graph::relation::{Linked, Relate};

fn relate(source: Related<String>, label: String, relation: String) -> Related<String> {
    let mut graph = source;
    graph.relate(&label, &relation);
    graph
}

fn descendants(source: Related<String>, from: String) -> Option<Vec<String>> {
    source
        .descendants(&from)
        .map(|set| set.into_iter().collect::<Vec<_>>())
}

fn predecessors(source: Related<String>, from: String) -> Option<Vec<String>> {
    source
        .predecessors(&from)
        .map(|set| set.into_iter().collect::<Vec<_>>())
}
