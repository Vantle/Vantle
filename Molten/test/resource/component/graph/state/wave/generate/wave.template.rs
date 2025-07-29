use component::graph::state::Wave;
use component::graph::traits::node::Queryable;

fn subset(candidate: Wave<String>, basis: Wave<String>) -> Option<Wave<String>> {
    candidate.subset(&basis).cloned()
}
