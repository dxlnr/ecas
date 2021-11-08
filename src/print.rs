pub fn print_single_row(lattice: &Vec<u8>) {
    /* print single lattice. */
    let mut output = String::new();
    for (_, y) in lattice.iter().enumerate() {
        if y == &1 {
            output += "\u{25A1} ";
        } else {
            output += "\u{25A0} ";
        }
    }
    println!("{}", output);
}

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
