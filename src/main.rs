use macroquad::prelude::*;

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

pub fn push_and_pop(vec: &Vec<Vec<u8>>, input: Vec<u8>) -> Vec<Vec<u8>> {
    /* Pushes an element to a vector and pops the first element. */
    let mut output = vec.to_vec();
    output.push(input);
    output.remove(0);
    output
}

// pub fn run(width: usize, iter: usize, rule: rules::Rule, init: Init) {
//     let mut lattice;
//     lattice = Init::initialize(&init, width);
//
//     for _ in 0..iter {
//         print::print_single_row(&lattice);
//         lattice = grid::cmp_next(&lattice, &rule);
//
//         /* add some sleeping time. */
//         let sleep_t = time::Duration::from_millis(10);
//         thread::sleep(sleep_t);
//     }
// }

pub async fn run(width: usize, iter: usize, rule: rules::Rule, init: Init) {
    let mut grid: Vec<Vec<u8>> = vec![vec![0; width]; iter];
    grid = push_and_pop(&grid, Init::initialize(&init, width));

    loop {
        clear_background(WHITE);
        //let s_grid = screen_width().min(screen_height());
        //let offset_x = (screen_width() - s_grid) / 2.;
        //let offset_y = (screen_height() - s_grid) / 2.;
        //let sq_size = (screen_height() - offset_y * 2.) / width as f32;
        let s_cell = screen_width() / width as f32;

        for r in 0..iter {
            for i in 0..grid[0].len() {
                draw_rectangle(
                    i as f32 * s_cell,
                    r as f32 * s_cell,
                    s_cell,
                    s_cell,
                    match grid[r][i] {
                        1 => BLACK,
                        0 => WHITE,
                        _ => WHITE,
                    },
                );
            }
        }
        grid = push_and_pop(&grid, grid::cmp_next(&grid[iter - 1], &rule));

        next_frame().await
    }
}

#[macroquad::main("ecas")]
async fn main() {
    run(250, 200, rules::Rule::R30, Init::Single).await;

    //run(150, 200, rules::Rule::R30, Init::Single);
}
