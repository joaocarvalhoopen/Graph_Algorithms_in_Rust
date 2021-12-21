/// Name: Graph algorithms in Rust
///
/// Author of the port: João Nuno Carvalho
/// 
/// Date: 2021.12.14
///  
/// Description: Port from Python to Rust of the original implementation from
///              the github repository:
/// 
///    GitHub - TheAlgorithms - Python
///    https://github.com/TheAlgorithms/Python/blob/master/DIRECTORY.md#graphs
/// 
/// License: MIT Open Source License, like the original license of the github
///          repository TheAlgorithms.
///    https://github.com/TheAlgorithms/Python/blob/master/DIRECTORY.md#graphs
/// 
/// (See project page)
///


// Module definition
// mod utils;
mod a_star;
mod articulation_points;

// Imports
use crate::a_star::AStar;
use crate::articulation_points::ArticulationPoints;

// Macros
// Macro visibility rules.
// https://users.rust-lang.org/t/3-things-that-the-rust-standard-library-should-have/68825/11
#[macro_use] mod utils;

// Load's the Macro from maplit crate, at this crate root.
#[macro_use] extern crate maplit;

fn main() {
    println!("********************************");
    println!("**  Graph Algorithms in Rust  **");
    println!("********************************");

    // A* (Star) Search Algorithm.
    AStar::main();

    // Finding Articulation Points (or Cut Vertices) in Undirected Graph.
    ArticulationPoints::main();
}


