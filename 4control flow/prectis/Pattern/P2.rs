/*
        *
      * *
    * * *
*/

fn main() {

    let row = 4;

    for i in 1 .. row {
        
        for _ in 0..(row-i) {
            print!(" ");
        }
        for _ in 0..i {
            
            print!("*");
        }
    println!();
    }
}