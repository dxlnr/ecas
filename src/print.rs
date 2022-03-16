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

pub fn print_grid(grid: &Vec<Vec<u8>>) {
    /* print 2D grid. */
    let mut output = String::new();
    for (_, x) in grid.iter().enumerate() {
        for (_, y) in x.iter().enumerate() {
            if y == &1 {
                output += "\u{25A1} ";
            } else {
                output += "\u{25A0} ";
            }
        }
        output += "\n";
    }
    println!("{}", output);
}

pub fn print_matrix(grid: &Vec<Vec<u8>>) {
    for (_, x) in grid.iter().enumerate() {
        println!("{:?}", x);
    }
}
