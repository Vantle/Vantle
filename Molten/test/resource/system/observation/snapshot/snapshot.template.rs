use assemble::Assemble;
use hypergraph::Hypergraph;
use state::{assembler, capture, restore};

fn roundtrip(graph: Hypergraph<usize>) -> bool {
    let snapshot = capture(&graph, "test").expect("capture failed");
    let restored: Hypergraph<usize> = restore(&snapshot).expect("restore failed");
    graph == restored
}

fn compressed(graph: Hypergraph<usize>) -> (bool, bool) {
    let uncompressed = assembler(&graph, "uncompressed")
        .compressed(false)
        .assemble()
        .expect("assemble failed");
    let compressed = assembler(&graph, "compressed")
        .compressed(true)
        .assemble()
        .expect("assemble failed");
    let smaller = compressed.state.len() <= uncompressed.state.len();
    let restored: Hypergraph<usize> = restore(&compressed).expect("restore failed");
    let valid = graph == restored;
    (smaller, valid)
}

fn trigger(graph: Hypergraph<usize>, name: String) -> String {
    let snapshot = capture(&graph, &name).expect("capture failed");
    snapshot.trigger
}

fn timestamp(graph: Hypergraph<usize>) -> bool {
    let snapshot = capture(&graph, "test").expect("capture failed");
    snapshot.timestamp > 0
}
