/*
    create non mut variable and store prefix value of 9 and incres one and print use shadowing
*/
fn main() {

    let r=9;
        println!("{}",r);

        let r = r+1;
            println!("{}",r);
         
            {
                let r = r * 2;
                    println!("{}",r);
            }

}