mod grid;
mod rules;

pub fn print_pretty_boxes(grid: &Vec<Vec<u8>>) -> String {
    let mut pretty_grid = "".to_string();
    for (_, x) in grid.iter().enumerate() {
        pretty_grid += "|";
        for (_, y) in x.iter().enumerate() {
            if y == &1 {
                pretty_grid += "\u{25A0}";
            } else {
                pretty_grid += "\u{25A1}";
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
<<<<<<< HEAD
    let mut grid = create_random_grid(10, 4);
    
    println!("\n\nthis is the random grid we start with:\n");
    print_matrix(&grid);
    print_pretty_boxes(&grid);

    println!("\n\nnow let's perform an iteration step !\n");
    grid = perform_step(grid);
    print_matrix(&grid);
    print_pretty_boxes(&grid);

=======
    let mut grid = grid::create_random_grid(64, 32);
    grid = grid::perform_step(grid);

    /* Printing section */
    print_pretty_boxes(&grid);
    let black_sq = "\u{25A0}";
    let white_sq = "\u{25A1}";
    println!("{}", black_sq);
    println!("{}", white_sq);
>>>>>>> origin/main
}
