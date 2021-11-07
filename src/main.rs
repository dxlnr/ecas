extern crate rand;
use rand::prelude::*;

mod rules;

// fn create_grid(width: usize, height: usize) {
//     /* Creates a dynamically-sized 2D grid. */
//     let mut grid_raw = vec![0; width * height];
//     let mut grid_base: Vec<_> = grid_raw.as_mut_slice().chunks_mut(width).collect();
//     let grid = grid_base.as_mut_slice();
// }

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

pub fn print_pretty_boxes(grid: &Vec<Vec<u8>>) -> String {
    let mut pretty_grid = "".to_string();
    for (_, x) in grid.iter().enumerate() {
        pretty_grid += "|";
        for (_, y) in x.iter().enumerate() {
            if y == &1 {
                pretty_grid += "x";
            }
            else {
                pretty_grid += " ";
            }
        }
        pretty_grid += "|\n";
    }
    println!("{}", pretty_grid);
    pretty_grid
}

pub fn print_matrix(grid: &Vec<Vec<u8>>) {
    for (_, x) in grid.iter().enumerate() {
        println!("{:?}", x);
    }
}

fn main() {
    let mut grid = create_random_grid(10, 4);
    
    println!("\n\nthis is the random grid we start with:\n");
    print_matrix(&grid);
    print_pretty_boxes(&grid);

    println!("\n\nnow let's perform an iteration step !\n");
    grid = perform_step(grid);
    print_matrix(&grid);
    print_pretty_boxes(&grid);

}
