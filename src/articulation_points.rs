/// Finding Articulation Points (or Cut Vertices) in Undirected Graph
///
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
/// 
/// License: MIT Open Source License, like the original license of the github
///          repository TheAlgorithms.
///    https://github.com/TheAlgorithms/Python/blob/master/DIRECTORY.md#graphs
/// 
/// (See project page)
///

// Imports
use std::collections::HashMap;

use crate::utils::graph;

// Finding Articulation Points (or Cut Vertices) in Undirected Graph.

type GraphAP = HashMap<usize, Vec<usize>>;

pub struct ArticulationPoints {

}

impl ArticulationPoints {

    pub fn compute_ap(l: & GraphAP) {
        let n = l.len();
        let mut out_edge_count;
        let mut low = vec![0_usize; n];
        let mut visited = vec![false; n];
        let mut is_art = vec![false; n];
    
        fn dfs(l: & GraphAP, root: usize, at: usize, parent: i32, mut out_edge_count: i32,
               visited: & mut Vec<bool>, low: & mut Vec<usize>, is_art: & mut Vec<bool>)
              -> i32 {
            if parent == root as i32 {
                out_edge_count += 1;
            }
            visited[at] = true;
            low[at] = at;

            for to in l.get(& at).unwrap() {
                if (*to) as i32 == parent {
                    continue;
                } else if !visited[*to] {
                    out_edge_count = dfs(l, root, *to, at as i32, out_edge_count, visited, low, is_art);
                    low[at] = usize::min(low[at], low[*to]);
                    // AP found via bridge
                    if at < low[*to] {
                        is_art[at] = true;
                    }
                    // AP found via cycle
                    if at == low[*to] {
                        is_art[at] = true;
                    }
                } else {
                    low[at] = usize::min(low[at], *to);
                }
            }
            
            out_edge_count
        }


        for i in 0..n {
            if !visited[i] {
                out_edge_count = 0;
                out_edge_count = dfs(l, i, i, -1, out_edge_count, & mut visited, & mut low, & mut is_art);
                is_art[i] = out_edge_count > 1;
            }
        }
    
        println!("\nArticulation Points:");
        for x in 0..is_art.len() {
            if is_art[x] == true {
                println!("{}", x);
            }
        }

    }

    pub fn main() {

        println!("\n\n\n*****************************************************************");
        println!("Finding Articulation Points (or Cut Vertices) in Undirected Graph");

        //*********
        // 1º Solution

        // Adjacency list of graph.
        let data_1: GraphAP = HashMap::from([
            (0, vec![1, 2]),
            (1, vec![0, 2]),
            (2, vec![0, 1, 3, 5]),
            (3, vec![2, 4]),
            (4, vec![3]),
            (5, vec![2, 6, 8]),
            (6, vec![5, 7]),
            (7, vec![6, 8]),
            (8, vec![5, 7]),
        ]);
        
        //*********
        // 2º Solution

        // Load's the Macro from maplit crate, at this crate root,
        // has to be on the main of the program.
        //
        // #[macro_use] extern crate maplit;        

        let data_2 = hashmap!{
            0 => vec![1, 2],
            1 => vec![0, 2],
            2 => vec![0, 1, 3, 5],
            3 => vec![2, 4],
            4 => vec![3],
            5 => vec![2, 6, 8],
            6 => vec![5, 7],
            7 => vec![6, 8],
            8 => vec![5, 7]
        };

        //*********
        // 3º Solution - The best one!
        //               The Macro is at the beginning of the file.

        let data_3 = graph![
            0 => 1, 2;
            1 => 0, 2;
            2 => 0, 1, 3, 5;
            3 => 2, 4;
            4 => 3;
            5 => 2, 6, 8;
            6 => 5, 7;
            7 => 6, 8;
            8 => 5, 7
        ];

        assert_eq!(data_1, data_2);
        assert_eq!(data_1, data_3);


        println!("\nAdjacency list of graph");
        for i in 0..data_1.len() {
            println!("({}, {:?})", i, data_1.get(& i).unwrap());
        }

        ArticulationPoints::compute_ap(& data_1);
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a_star() {

    }
 
}

