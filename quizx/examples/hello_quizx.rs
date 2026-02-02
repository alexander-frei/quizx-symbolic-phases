use quizx::basic_rules::{check_pivot, pivot_unchecked};
use quizx::graph::*;
use quizx::vec_graph::Graph;

fn main() {
    // Build a simple graph where pivoting can apply:
    //
    //     v2
    //      |H
    //  v0--H--v1   (v0 and v1 are the pivot pair, both Pauli phase)
    //      |H
    //     v3
    //
    // v0 and v1 are Z-spiders with phase 0 (Pauli), connected by H-edge
    // v2 and v3 are Z-spiders connected to v0 and v1 by H-edges


    let mut g = Graph::new();

    // Add the two pivot vertices (Pauli Z-spiders with phase 0)
    let v0 = g.add_vertex(VType::Z); // phase defaults to 0
    let v1 = g.add_vertex(VType::Z);
    let v2 = g.add_vertex(VType::Z);
    let v3 = g.add_vertex(VType::Z);

    // Connect v0 -- v1 with Hadamard edge
    g.add_hadamard_edge(v0, v1);
    g.add_hadamard_edge(v0, v2);
    g.add_hadamard_edge(v0, v3);
    g.add_hadamard_edge(v1, v3);


    println!("Before pivoting:\n {}", g.to_dot());


    if check_pivot(&g, v0, v1) {
        pivot_unchecked(&mut g, v0, v1);
        println!("After pivoting:\n {}", g.to_dot());
    }
}