/*
    store value in to a variable and comper and print to unother function
*/
fn main() {

    let x = 10; 
    let y = 20;
    let c:str="ronak";

        print(x,y);

}
fn print(a:i32,b:i32,name:&str) {

        if a < b {
                println!("B is big");
        }
        else if a > b {
            println!("A is big");
        }
        else  {
                println!("A AND B IS EQUAL");
        }

        println!("{}",name);
}