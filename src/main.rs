extern crate rand;
use rand::prelude::*;

mod rules;

// fn create_grid(width: usize, height: usize) {
//     /* Creates a dynamically-sized 2D grid. */
//     let mut grid_raw = vec![0; width * height];
//     let mut grid_base: Vec<_> = grid_raw.as_mut_slice().chunks_mut(width).collect();
//     let grid = grid_base.as_mut_slice();
// }

pub fn create_random_grid(width: usize, height: usize) -> Vec<Vec<i32>> {
    /* Creates a randomly initiallized 2D grid. */
    let mut rng = rand::thread_rng();
    let mut grid: Vec<Vec<i32>> = vec![vec![0; width]; height];
    for i in 0..height {
        for j in 0..width {
            grid[i][j] = rng.gen_range(0..2);
        }
    }
    
    // println!("{:?}", grid);
    for (i, x) in grid.iter().enumerate() {
        println!("{:?}", x);
    }

    grid
}

fn main() {
    //rules::rule_30();
    create_random_grid(10, 4);
}
