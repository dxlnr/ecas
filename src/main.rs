mod grid;
mod rules;

fn main() {
    let mut grid = grid::create_random_grid(64, 32);
    grid = grid::perform_step(grid);

    /* Printing section */
    for (_, x) in grid.iter().enumerate() {
        println!("{:?}", x);
    }
}
