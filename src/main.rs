mod grid;
mod rules;
use std::{thread, time};

pub fn print_pretty_boxes(grid: &Vec<Vec<u8>>) {
    let mut pretty_grid = "".to_string();
    for (_, x) in grid.iter().enumerate() {
        pretty_grid += "|";
        for (_, y) in x.iter().enumerate() {
            if y == &1 {
                pretty_grid += "\u{25A1} ";
            } else {
                pretty_grid += "\u{25A0} ";
            }
        }
        pretty_grid += "|\n";
    }
    println!("{}", pretty_grid);
}

pub fn print_matrix(grid: &Vec<Vec<u8>>) {
    for (_, x) in grid.iter().enumerate() {
        println!("{:?}", x);
    }
}

pub fn run(width: usize, iter: usize, rule: rules::Rule) {
    let mut grid: Vec<Vec<u8>> = Vec::new();

    grid.push(grid::create_random_lattice(width));

    for i in 0..iter {
        print_pretty_boxes(&grid);
        grid.push(grid::cmp_next(&grid[i], &rule));

        /* add some sleeping time. */
        let sleep_t = time::Duration::from_millis(100);
        thread::sleep(sleep_t);
    }
    print_pretty_boxes(&grid);
}

fn main() {
    run(75, 600, rules::Rule::R30);
}
