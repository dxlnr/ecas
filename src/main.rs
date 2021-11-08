mod grid;
mod print;
mod rules;
use std::{thread, time};

pub fn run(width: usize, iter: usize, rule: rules::Rule) {
    let mut lattice;

    //grid.push(grid::create_random_lattice(width));
    lattice = grid::sierpinski_triangle_initialization(width);

    for _ in 0..iter {
        print::print_single_row(&lattice);
        lattice = grid::cmp_next(&lattice, &rule);

        /* add some sleeping time. */
        let sleep_t = time::Duration::from_millis(100);
        thread::sleep(sleep_t);
    }
}

fn main() {
    run(300, 1000, rules::Rule::R188);
}
