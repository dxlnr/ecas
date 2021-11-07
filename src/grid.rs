extern crate rand;
use rand::prelude::*;

use crate::rules;

// fn create_grid(width: usize, height: usize) {
//     /* Creates a dynamically-sized 2D grid. */
//     let mut grid_raw = vec![0; width * height];
//     let mut grid_base: Vec<_> = grid_raw.as_mut_slice().chunks_mut(width).collect();
//     let grid = grid_base.as_mut_slice();
// }

fn to_u64(slice: &[u64]) -> u64 {
    slice.iter().fold(0, |acc, &b| acc * 2 + b as u64)
}

fn to_u8(slice: &[u8]) -> u8 {
    slice.iter().fold(0, |acc, &b| acc * 2 + b as u8)
}

pub fn create_random_grid(width: usize, height: usize) -> Vec<Vec<u8>> {
    /* Creates a randomly initiallized 2D grid. */
    let mut rng = rand::thread_rng();
    let mut grid: Vec<Vec<u8>> = vec![vec![0; width]; height];
    for i in 0..height {
        for j in 0..width {
            grid[i][j] = rng.gen_range(0..2);
        }
    }
    grid
}

pub fn perform_step(mut grid: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    /* Performs one step of specific rule to alter the grid. */
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if j == 0 {
                grid[i][j] = rules::rule_30_u8(to_u8(&[0, grid[i][0], grid[i][1]]));
            } else if j == (grid[0].len() - 1) {
                grid[i][j] = rules::rule_30_u8(to_u8(&[grid[i][j - 1], grid[i][j], 0]));
            } else {
                grid[i][j] = rules::rule_30_u8(to_u8(&grid[i][(j - 1)..(j + 2)].to_vec()));
            }
        }
    }
    grid
}
