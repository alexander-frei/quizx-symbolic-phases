use quizx::basic_rules::{check_pivot, pivot_unchecked};
use quizx::graph::*;
use quizx::vec_graph::Graph;

fn main() {
    println!("=== Pivoting Rule Example ===\n");

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

    // Add two more Z-spiders as neighbors
    let v2 = g.add_vertex(VType::Z);
    let v3 = g.add_vertex(VType::Z);

    // Connect v0 -- v1 with Hadamard edge
    g.add_edge_with_type(v0, v1, EType::H);

    // Connect v0 to v2 and v3 with Hadamard edges
    g.add_edge_with_type(v0, v2, EType::H);
    g.add_edge_with_type(v0, v3, EType::H);

    // Connect v1 to v2 and v3 with Hadamard edges
    g.add_edge_with_type(v1, v2, EType::H);
    g.add_edge_with_type(v1, v3, EType::H);

    println!("BEFORE pivot:");
    println!("  Vertices: {}", g.num_vertices());
    println!("  Edges: {}", g.num_edges());
    println!("  v0 phase: {:?}, v1 phase: {:?}", g.phase(v0), g.phase(v1));
    println!("  v0 neighbors: {:?}", g.neighbor_vec(v0));
    println!("  v1 neighbors: {:?}", g.neighbor_vec(v1));
    println!("  v2 neighbors: {:?}", g.neighbor_vec(v2));
    println!("  v3 neighbors: {:?}", g.neighbor_vec(v3));

    // Check if pivot applies
    let can_pivot = check_pivot(&g, v0, v1);
    println!("\n  Can pivot at (v0, v1)? {}", can_pivot);

    if can_pivot {
        println!("\nApplying pivot at (v0={}, v1={})...", v0, v1);
        pivot_unchecked(&mut g, v0, v1);

        println!("\nAFTER pivot:");
        println!("  Vertices: {}", g.num_vertices());
        println!("  Edges: {}", g.num_edges());
        println!("  Remaining vertices: {:?}", g.vertices().collect::<Vec<_>>());

        // v0 and v1 should be removed, v2 and v3 should now be connected
        if g.vertex_data_opt(v2).is_some() {
            println!("  v2 neighbors: {:?}", g.neighbor_vec(v2));
            println!("  v2 phase: {:?}", g.phase(v2));
        }
        if g.vertex_data_opt(v3).is_some() {
            println!("  v3 neighbors: {:?}", g.neighbor_vec(v3));
            println!("  v3 phase: {:?}", g.phase(v3));
        }
    }
}