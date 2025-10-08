use rand::prelude::*;
use std::io;

    fn main (){
        println!("Insert a value from 1 to 5");
        let mut yy = String::new();
        io::stdin().read_line(&mut yy).expect("msg");

            let y = yy.trim().parse().expect("msg");

        let mut random = rand::rng();
        let x = random.random_range(1..=5);

        if x!=y {
            println!("You missed")
        } else {
            println!("Congratulations you got it right, the number is {}", x);
        }
    }