/// A* (star) search algorithm
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
///    Wikipedia
///    https://en.wikipedia.org/wiki/A*_search_algorithm
/// 
/// License: MIT Open Source License, like the original license of the github
///          repository TheAlgorithms.
///    https://github.com/TheAlgorithms/Python/blob/master/DIRECTORY.md#graphs
/// 
/// (See project page)
///

// Imports
use crate::utils::Grid2D;

const DIRECTIONS: [[i32; 2]; 4] = [
    [-1, 0],  // left
    [0, -1],  // down
    [1, 0],   // right
    [0, 1],   // up
];

pub struct AStar { 

}

impl AStar {
    /// Function to search the path.
    pub fn search(
        grid: & mut Grid2D,
        init: (i32, i32),
        goal: (i32, i32),
        cost: i32,
        heuristic: & Grid2D,
        ) -> Result< (Vec<(i32, i32)>, Grid2D), String> {

        // The reference grid.
        let mut closed: Grid2D = Grid2D::new(grid.rows_len(), grid.cols_len());
        closed.set(init.0,  init.1, 1);
        // The action grid. 
        let mut action: Grid2D = Grid2D::new_with_val(grid.rows_len(), grid.cols_len(), 8);

        let mut x = init.0;
        let mut y = init.1;
        let mut g = 0;
        // cost from starting cell to destination cell
        let f = g + heuristic.get(x, y);
        let mut cell: Vec<(i32, i32, i32, i32)> = Vec::new();
        cell.push( (f, g, x, y) );
            
        // Flag that is set when search is complete.
        let mut found = false;
        // Flag set if we can't find expand.
        /*let mut resign = false;*/

        while !found /*&& !resign*/ {
            if cell.len() == 0 {
                return Err("Algorithm is unable to find solution".to_string());
            } else {
                // To choose the least costliest action so as to move closer to the goal.
                cell.sort();
                cell.reverse();
                let next = cell.pop().unwrap();
                x = next.2;
                y = next.3;
                g = next.1;

                if x == goal.0 && y == goal.1 {
                    found = true;
                } else {
                    // To try out different valid actions.
                    for i in 0..DIRECTIONS.len() {
                        let x2 = x + DIRECTIONS[i][0];
                        let y2 = y + DIRECTIONS[i][1];
                        if    x2 >= 0 && x2 < grid.rows_len() 
                           && y2 >= 0 && y2 < grid.cols_len() {
                            
                            if closed.get(x2, y2) == 0 && grid.get(x2, y2) == 0 {
                                let g2 = g + cost;
                                let f2 = g2 + heuristic.get(x2, y2);
                                cell.push( (f2, g2, x2, y2) );
                                closed.set(x2, y2, 1);
                                action.set(x2, y2, i as i32);
                            }
                        }
                    }
                }
            }
        }

        let mut invpath: Vec<(i32, i32)> = Vec::new();
        let mut x = goal.0;
        let mut y = goal.1;
        // We get the reverse path from here.
        invpath.push( (x, y) );
        while x != init.0 || y != init.1 {
            let x2 = x - DIRECTIONS[action.get(x, y) as usize][0];
            let y2 = y - DIRECTIONS[action.get(x, y) as usize][1];
            x = x2;
            y = y2;
            invpath.push( (x, y) );
        }

        let mut path = Vec::new();
        for i in 0..invpath.len() {
            path.push(invpath[invpath.len() - 1 - i]);
        }

        Ok( (path, action) )
    }

    pub fn main() {
        println!("\n\n\n***************************");
        println!("A* (Star) Search Algorithm.");

        // 0 are free path whereas 1's are obstacles.
        const ROWS_LEN: i32 = 5;
        const COLS_LEN: i32 = 6;
        let grid_ar = [
            0, 1, 0, 0, 0, 0,
            0, 1, 0, 0, 0, 0,
            0, 1, 0, 0, 0, 0,
            0, 1, 0, 0, 1, 0,
            0, 0, 0, 0, 1, 0,
        ];
        let mut grid = Grid2D::new_ar(ROWS_LEN, COLS_LEN, &grid_ar[..]);

        println!("\nThe board, 0 are free path whereas 1's are obstacles.");
        grid.print();

        // All coordinates are given in format [y,x] .
        let init = (0, 0);
        let goal = (grid.rows_len() - 1, grid.cols_len() - 1);
        let cost = 1;

        // The cost map which pushes the path closer to the goal.
        let mut heuristic = Grid2D::new(grid.rows_len(), grid.cols_len());
        
        for i in 0..heuristic.rows_len() {
            for j in 0..heuristic.cols_len() {
                let val = i32::abs(i - goal.0) + i32::abs(j - goal.1);
                heuristic.set(i, j, val);
                if grid.get(i, j) == 1 {
                    // Added extra penalty in the heuristic map.
                    heuristic.set(i, j, 99);
                }
            }
        }

        println!("The cost map.");
        heuristic.print();

        let res  = AStar::search(& mut grid, init, goal, cost, & heuristic);

        if let Err(e) = res {
            println!("{}", e);
            return;
        }

        let (path, action) = res.unwrap();

        println!("Action Map, 8 is the initialization value.");
        action.print();
    
        println!("Path");
        for elem in path {
            println!("({}, {}), ", elem.0, elem.1);
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a_star() {

    }
 
}

