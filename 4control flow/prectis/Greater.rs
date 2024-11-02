/*
Greater Number: Write a function greater_number that takes two integers and 
returns the larger of the two using an if-else statement.
*/
fn greater_number(x:i32,y:i32)->i32{

        if x > y {
            x
        }
        else {
            y
        }

        
}
fn main() {

        let a=3;
        let b=4;

            let r = greater_number(a,b);

        println!("{} is big ",r);

        
}