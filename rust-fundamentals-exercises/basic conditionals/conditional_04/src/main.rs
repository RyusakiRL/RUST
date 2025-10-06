use std::io;

fn main () {
    println!("Write a number");
        let mut number_str = String::new();
        io::stdin().read_line(&mut number_str)
        .expect("Enter a valid number");

            let number: f64 = number_str.trim().parse().expect("msg");
            
        if number%2.0 == 0.0 {

            println!("Your number is even");
            
        } else {
            
            println!("Your number is odd");

        }


}