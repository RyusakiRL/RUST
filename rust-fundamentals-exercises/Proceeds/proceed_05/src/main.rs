use std::io;

fn adder (numberone: i32, numbertwo: i32, sum: i32) {

    println!("The sum of {} + {} = {}", numberone, numbertwo, sum);

}


fn main () {
    println!("--Insert the first number--");
        let mut first_str = String::new();
        io::stdin().read_line(&mut first_str).expect("invalid number");
            let first: i32 = first_str.trim().parse().expect("Incorrect number");
    println!("--Insert the second number--");
        let mut second_str = String::new();
        io::stdin().read_line(&mut second_str).expect("Invalid number");
            let second: i32 = second_str.trim().parse().expect("Invalid number");
            let summer = first+second;

    adder(first, second, summer);
}