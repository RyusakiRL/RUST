use std::io;

fn bigger (numberone: i32, numbertwo: i32) {

    if numberone>numbertwo {
        println!("The number {} is bigger", numberone);
    }  else {
        println!("The number {} is bigger", numbertwo);
    }

}

fn main () {

    println!("---Insert the First number---");
        let mut first_str = String::new();
        io::stdin().read_line(&mut first_str).expect("Invalid number");
            let first: i32 = first_str.trim().parse().expect("Incorrect value");

    println!("---Insert the Second Number---");
        let mut second_str = String::new();
        io::stdin().read_line(&mut second_str).expect("Incorrect number");
            let second: i32 = second_str.trim().parse().expect("Invalid value");

    bigger(first, second);

}