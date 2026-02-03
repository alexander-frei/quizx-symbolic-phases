use quizx::simplify::full_simp;
use quizx::graph::*;
use quizx::vec_graph::Graph;
use quizx::circuit::Circuit;



fn main() {
    
    

    //------------------------------
    //   Random Clifford+T circuit
    //------------------------------
    let circuit = Circuit::random()
        .qubits(10)
        .depth(200)
        .clifford_t(0.3)
        .build();
    //------------------------------
    //   Converting to graph
    //------------------------------
    let mut graph: Graph = circuit.to_graph();
    //------------------------------
    //   Converting spiders
    //------------------------------
    graph.x_to_z();
    //------------------------------
    //   Setting Z type inputs
    //------------------------------
    for &v in graph.inputs().clone().iter() {
        graph.set_vertex_type(v, VType::Z);
    }
    //------------------------------
    //   Checking random circuit
    //------------------------------
    // println!("Random circuit:\n{}", graph.to_dot());
    
    
    //------------------------------
    //   Clifford simplification
    //------------------------------
    full_simp(&mut graph);
    //------------------------------
    //   Simplified graph
    //------------------------------
    println!("Simplified graph:\n{}", graph.to_dot());

}