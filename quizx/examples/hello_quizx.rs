use quizx::basic_rules::{check_pivot, pivot_unchecked};
use quizx::graph::*;
use quizx::vec_graph::Graph;
use quizx::circuit::Circuit;



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



    // Random Clifford+T circuit
    let circuit = Circuit::random()
        .qubits(5)
        .depth(10)
        .p_cz(0.3)      // 30% CZ gates
        .p_h(0.3)       // 30% H gates
        .clifford_t(0.1)   // 10% T gates, rest split among CNOT/H/S
        .seed(42)          // optional: reproducible
        .build();


    // Convert to graph
    let mut graph: Graph = circuit.to_graph();
    graph.x_to_z();
    println!("{}", graph.to_dot());
    

    let mut g = Graph::new();
    let v0 = g.add_vertex(VType::Z); // phase defaults to 0
    let v1 = g.add_vertex(VType::Z);
    let v2 = g.add_vertex(VType::Z);
    let v3 = g.add_vertex(VType::Z);

    g.add_hadamard_edge(v0, v1);
    g.add_hadamard_edge(v0, v2);
    g.add_hadamard_edge(v0, v3);
    g.add_hadamard_edge(v1, v3);


    println!("Before pivoting:\n {}", g.to_dot());
    if check_pivot(&g, v0, v1) {
        pivot_unchecked(&mut g, v0, v1);
    }
    println!("After pivoting:\n {}", g.to_dot());


}