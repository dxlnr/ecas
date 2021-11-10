extern crate rand;
use rand::prelude::*;

use crate::rules;

// fn create_grid(width: usize, height: usize) {
//     /* Creates a dynamically-sized 2D grid. */
//     let mut grid_raw = vec![0; width * height];
//     let mut grid_base: Vec<_> = grid_raw.as_mut_slice().chunks_mut(width).collect();
//     let grid = grid_base.as_mut_slice();
// }

fn to_u8(slice: &[u8]) -> u8 {
    slice.iter().fold(0, |acc, &b| acc * 2 + b as u8)
}

pub fn create_random_lattice(length: usize) -> Vec<u8> {
    /* Creates a randomly initiallized 1D lattice of a set length. */
    let mut rng = rand::thread_rng();
    let mut lattice: Vec<u8> = vec![0; length];
    for i in 0..length {
        lattice[i] = rng.gen_range(0..2);
    }
    lattice
}

pub fn sierpinski_triangle_init(length: usize) -> Vec<u8> {
    /* Inits a lattice with all zeros and a one in the center. */
    let mut lattice: Vec<u8> = vec![0; length];
    lattice[length / 2] = 1;
    lattice
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

pub fn perform_step(grid: &Vec<Vec<u8>>, rule: &rules::Rule) -> Vec<Vec<u8>> {
    /* Performs one step of specific rule to alter the grid. */
    let mut update = grid.clone();
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if j == 0 {
                update[i][j] = rules::Rule::apply_rule(rule, to_u8(&[0, grid[i][0], grid[i][1]]));
            } else if j == (grid[0].len() - 1) {
                update[i][j] =
                    rules::Rule::apply_rule(rule, to_u8(&[grid[i][j - 1], grid[i][j], 0]));
            } else {
                update[i][j] =
                    rules::Rule::apply_rule(rule, to_u8(&grid[i][(j - 1)..(j + 2)].to_vec()));
            }
        }
    }
    update
}

pub fn cmp_next(lattice: &Vec<u8>, rule: &rules::Rule) -> Vec<u8> {
    /* Computes the next lattice. */
    let mut update = lattice.clone();
    for i in 0..lattice.len() {
        if i == 0 {
            update[i] = rules::Rule::apply_rule(
                rule,
                to_u8(&[*lattice.last().unwrap(), lattice[0], lattice[1]]),
            );
        } else if i == (lattice.len() - 1) {
            update[i] =
                rules::Rule::apply_rule(rule, to_u8(&[lattice[i - 1], lattice[i], lattice[0]]));
        } else {
            update[i] = rules::Rule::apply_rule(rule, to_u8(&lattice[(i - 1)..(i + 2)].to_vec()));
        }
    }
    update
}
