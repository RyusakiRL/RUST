use std::io;

fn even (number_integer: i32) {

if number_integer%2 == 0 {
    println!("The number {} is pair", number_integer);
} else {
    println!("The number {} is odd", number_integer);
}
}

fn main () {

    println!("--Insert the number--");
        let mut number_str = String::new();
        io::stdin().read_line(&mut number_str).expect("Invalid number");
            let number: i32 = number_str.trim().parse().expect("Not a correct value");
    
    even(number);

}