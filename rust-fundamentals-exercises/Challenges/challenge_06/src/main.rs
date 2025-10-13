use rand::prelude::*;
use std::io;

fn main () {

    let mut random_thread = rand::rng();
    let randomzer = random_thread.random_range(1..=10);
    let mut limit = 0;

while limit<4 {
     
     println!("--Insert a Number--");
        let mut number_str = String::new();
            io::stdin().read_line(&mut number_str).expect("msg");
            let number: i32 = number_str.trim().parse().expect("msg");
        
        limit = limit + 1;
 
    if number == randomzer {
        println!("You won"); break;
    } 
    
    if number != randomzer && limit<3 {
        println!("Try again");
    }        

    if limit == 4 {
        println!("[You lost]");
    }
   
}

    

}