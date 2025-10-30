use std::io;
use num_primes::{BigUint, Verification};

fn main () {
    println!("---INSERT THE POSITION OF PRIME NUMBER---");
        let mut position_str = String::new();
        io::stdin().read_line(&mut position_str).expect("Invalid value");
        let position: usize = position_str.trim().parse().expect("Error the value is invalid");
    
        let mut count = 0usize;
        let mut num = BigUint::from(2u32);

    loop {
        if Verification::is_prime(&num) {
            count += 1;
            if count == position-1 {
                println!("---RESULTS---");
                println!("The prime number in the position {} is [{}]", position, num);
                break;
            }
        }
        num += 1u32;
    }

}