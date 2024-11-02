/*
Write a function add_three_numbers that takes three integers as parameters and returns their sum.
*/

fn main(){

        let a = 3; let b = 4; let c = 8; //here is three variable with prefix value
    
           let sum = add_three_numbers(a,b,c);

            
                println!("{}",sum);
}
fn add_three_numbers(x:i32,y:i32,z:i32) ->i32{ // return  -> i32 

    let sum = x + y + z;
           
}