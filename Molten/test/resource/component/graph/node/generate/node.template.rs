use component::graph::node::Node;
use component::graph::traits::node::Queryable;

fn disjoint(candidate: Node<String>, basis: Node<String>) -> Option<Node<String>> {
    candidate.disjoint(&basis).cloned()
}

fn isomorphic(candidate: Node<String>, basis: Node<String>) -> Option<Node<String>> {
    candidate.isomorphic(&basis).cloned()
}

fn joint(candidate: Node<String>, basis: Node<String>) -> Option<Node<String>> {
    candidate.joint(&basis).cloned()
}

fn subset(candidate: Node<String>, basis: Node<String>) -> Option<Node<String>> {
    candidate.subset(&basis).cloned()
}

fn superset(candidate: Node<String>, basis: Node<String>) -> Option<Node<String>> {
    candidate.superset(&basis).cloned()
}
