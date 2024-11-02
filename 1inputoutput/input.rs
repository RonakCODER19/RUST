/*
    take input to the user and print
*/
use std::io;  
fn main() {

    println!("Ente your name :");
    let mut name = String::new(); // create input variable
    io::stdin() //call input libary
 
        .read_line(&mut name) // stor value in to a variable
        .expect("");

    println!("hey ! hii how are you {name}"); // display value 
}
