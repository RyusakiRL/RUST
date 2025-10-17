use std::io;

fn natalinda (numberone: i32, numbertwo: i32) -> i32 {
    numberone+numbertwo
}
fn main () {

    println!("--Insert the first number--");
        let mut first_str = String::new();
        io::stdin().read_line(&mut first_str).expect("Invalid number");
            let first: i32 = first_str.trim().parse().expect("Invalid value");

    println!("--Insert the second number--");
        let mut second_str = String::new();
        io::stdin().read_line(&mut second_str).expect("Invalid number");
            let second: i32 = second_str.trim().parse().expect("Invalid value");

            let summ: i32 = natalinda(first, second);

    println!("{} + {} = {}", first, second, summ);
    
}