fn main(){

    let y = 5;
    let z = 'h';
    print(y);  // pass value of a y variable in to print function

        print2(y,z); //pass two values y and z in to print2 function
}
fn print(x : i32) {  // x is a paramiter i32 is a data type of variable to a print function

    println!("{}",x);
}
fn print2(x : i32, c : char) {  // x is a paramiter i32 is a data type of variable y c is second peramiter and char is a type of c variable

        println!("{}{}",x,c);
}