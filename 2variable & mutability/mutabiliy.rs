/*
    create mut variable and print && overshadowing
*/
fn main() {

    // shadowing
        let x = 5; 

            let x = x * 2;
                println!("{x}");

    // mutability variable
        let mut y = 5;

            y = y * 2;
                println!("{y}");
}