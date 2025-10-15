use std::io;
use rand::prelude::*;

fn main () {

    let mut vectorios = Vec::new();
    let mut rng = rand::rng();
    let mut repeated = 0;

    for _ in 0..=29 {
        
        let mut random = rng.random_range(1..=15);

        vectorios.push(random);
    }

        println!("Insert a number from one to fifteen");
            let mut number_str = String::new();
            io::stdin().read_line(&mut number_str).expect("Invalid number");
                let number: i32 = number_str.trim().parse().expect("Invalid number");

        for i in 0..=29 {
        
        if number == vectorios[i] {
            println!("The position of number is [{}]", i);
        repeated= repeated + 1;
        
        }
        
        }
    
         println!("The number repeated {} times", repeated);
}