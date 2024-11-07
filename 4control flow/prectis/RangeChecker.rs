/*
     Write a function in_range that takes an integer and checks if it’s between 1 and 100 (inclusive). 
     Return true if it is, otherwise false.
*/
fn in_range(n:i32)->bool {

        if n>=1 && n<=100 {
            true
        }
        else {
            
            false
        }

}
fn main() {

        let number = 120;

          let a =in_range(number);
          println!("{}",a);
         if a {

                println!("{} is is in the range of 1 to 100",number);
        }
        else {

                println!("{} is not in the range of 1 to 100",number);
        }

        
}