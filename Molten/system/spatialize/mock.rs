use std::collections::{BTreeMap, BTreeSet};

use hypergraph::{Edge, Hypergraph, Label, Meta, Node};
use particle::Particle;
use relation::Edge as RelationEdge;
use wave::Wave;

fn particle(elements: &[&str]) -> Particle<String> {
    let mut map = BTreeMap::new();
    for &elem in elements {
        *map.entry(elem.to_string()).or_insert(0) += 1;
    }
    Particle { elements: map }
}

fn wave(elements: &[&str]) -> Wave<String> {
    Wave {
        particles: BTreeMap::from([(particle(elements), 1)]),
    }
}

#[must_use]
pub fn graph() -> Hypergraph<String> {
    let nodes = BTreeSet::from([
        Node {
            label: Label(0),
            particle: particle(&["True"]),
        },
        Node {
            label: Label(1),
            particle: particle(&["False"]),
        },
        Node {
            label: Label(2),
            particle: particle(&["And", "True", "True"]),
        },
        Node {
            label: Label(3),
            particle: particle(&["Scope.1", "True", "True"]),
        },
        Node {
            label: Label(4),
            particle: particle(&["Not", "True"]),
        },
        Node {
            label: Label(5),
            particle: particle(&["Scope.3", "True"]),
        },
    ]);

    let edges = BTreeSet::from([
        Edge {
            label: Label(100),
            inference: RelationEdge {
                source: BTreeSet::from([Label(2)]),
                sink: BTreeSet::from([Label(3)]),
            },
            relation: RelationEdge {
                source: wave(&["And", "Boolean", "Boolean"]),
                sink: wave(&["Scope.1"]),
            },
        },
        Edge {
            label: Label(101),
            inference: RelationEdge {
                source: BTreeSet::from([Label(3)]),
                sink: BTreeSet::from([Label(0)]),
            },
            relation: RelationEdge {
                source: wave(&["Scope.1", "True", "True"]),
                sink: wave(&["True"]),
            },
        },
        Edge {
            label: Label(102),
            inference: RelationEdge {
                source: BTreeSet::from([Label(4)]),
                sink: BTreeSet::from([Label(5)]),
            },
            relation: RelationEdge {
                source: wave(&["Not", "Boolean"]),
                sink: wave(&["Scope.3"]),
            },
        },
        Edge {
            label: Label(103),
            inference: RelationEdge {
                source: BTreeSet::from([Label(5)]),
                sink: BTreeSet::from([Label(1)]),
            },
            relation: RelationEdge {
                source: wave(&["Scope.3", "True"]),
                sink: wave(&["False"]),
            },
        },
    ]);

    let mut future = BTreeMap::<Label, BTreeSet<Label>>::new();
    let mut past = BTreeMap::<Label, BTreeSet<Label>>::new();

    future.insert(Label(2), BTreeSet::from([Label(100)]));
    future.insert(Label(3), BTreeSet::from([Label(101)]));
    future.insert(Label(4), BTreeSet::from([Label(102)]));
    future.insert(Label(5), BTreeSet::from([Label(103)]));

    past.insert(Label(0), BTreeSet::from([Label(101)]));
    past.insert(Label(1), BTreeSet::from([Label(103)]));
    past.insert(Label(3), BTreeSet::from([Label(100)]));
    past.insert(Label(5), BTreeSet::from([Label(102)]));

    Hypergraph {
        meta: Meta {},
        nodes,
        edges,
        particles: 6,
        refractions: BTreeMap::from([
            (Label(0), Label(0)),
            (Label(1), Label(1)),
            (Label(2), Label(2)),
            (Label(3), Label(3)),
            (Label(4), Label(4)),
            (Label(5), Label(5)),
        ]),
        world: BTreeMap::from([
            (Label(0), 0),
            (Label(1), 0),
            (Label(2), 0),
            (Label(3), 0),
            (Label(4), 0),
            (Label(5), 0),
        ]),
        worlds: 1,
        united: BTreeMap::from([
            (Label(0), BTreeSet::from([Label(0)])),
            (Label(1), BTreeSet::from([Label(1)])),
            (Label(2), BTreeSet::from([Label(2)])),
            (Label(3), BTreeSet::from([Label(3)])),
            (Label(4), BTreeSet::from([Label(4)])),
            (Label(5), BTreeSet::from([Label(5)])),
        ]),
        future,
        past,
    }
}
