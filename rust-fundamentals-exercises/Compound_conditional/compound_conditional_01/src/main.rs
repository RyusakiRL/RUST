use std::io;

    fn main(){
        println!("Write the first number");
            let mut first_str = String::new();
            io::stdin().read_line(&mut first_str)
            .expect("Insert a valid value");

        println!("Write the second number");
            let mut second_str = String::new();
            io::stdin().read_line(&mut second_str)
            .expect("Insert a valid value");

                let first: f64 = first_str.trim().parse().expect("msg");
                let second: f64 = second_str.trim().parse().expect("msg");

        if first>second {
            println!("The first number is bigger");
            
        } else if first == second {
            println!("The first and second number is equal");
        } else {
            println!("The second number is bigger");
        }
    }