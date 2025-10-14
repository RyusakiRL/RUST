use std::io;
 
fn main () {

    println!("Insert the limit");
        let mut limit_str = String::new();
        io::stdin().read_line(&mut limit_str).expect("Insert a valuable number");
            let limit: u32 = limit_str.trim().parse().expect("Incorrect number");

        for i in 0..=limit {
            println!("[{}]", i);
        }

        println!("END!!");

}
