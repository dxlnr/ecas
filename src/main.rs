mod grid;
mod print;
mod rules;
use std::{thread, time};

pub enum Init {
    Single,
    Random,
}

impl Init {
    pub fn initialize(init: &Init, width: usize) -> Vec<u8> {
        match init {
            Init::Single => grid::sierpinski_triangle_init(width),
            Init::Random => grid::create_random_lattice(width),
        }
    }
}

pub fn run(width: usize, iter: usize, rule: rules::Rule, init: Init) {
    let mut lattice;
    lattice = Init::initialize(&init, width);

    for _ in 0..iter {
        print::print_single_row(&lattice);
        lattice = grid::cmp_next(&lattice, &rule);

        /* add some sleeping time. */
        let sleep_t = time::Duration::from_millis(10);
        thread::sleep(sleep_t);
    }
}

fn main() {
    run(300, 1000, rules::Rule::R184, Init::Single);
}
