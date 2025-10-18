use std::io;

fn potential (first: i32, second: i32) {
    let mut exponentiation = first;
    for _ in 1..second {
        exponentiation = exponentiation*first;
    }

    println!("---RESULTS---");
    println!("The result is {}", exponentiation);
}


fn main () {

    println!("--Insert the base of exponentiation--");
        let mut firstnumber_str = String::new();
        io::stdin().read_line(&mut firstnumber_str).expect("Invalid number");
            let firsto: i32 = firstnumber_str.trim().parse().expect("Invalid value");

    println!("--Insert the exponent--");
        let mut secondnumber_str = String::new();
        io::stdin().read_line(&mut secondnumber_str).expect("Invalid number");
            let secondo: i32 = secondnumber_str.trim().parse().expect("Invalid value");
        
    potential(firsto, secondo);
}