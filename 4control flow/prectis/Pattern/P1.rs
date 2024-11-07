fn print_triangle(rows: usize) {
    for i in 1..=rows {
        // Print spaces for alignment
        for _ in 0..(rows - i) {
            print!(" ");
        }
        // Print stars
        for _ in 0..i {
            print!("*");
        }
        // Move to the next line
        println!();
    }
}

fn main() {
    let rows = 4;
    print_triangle(rows);
}

