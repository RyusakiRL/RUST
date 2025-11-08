use std::io;

fn collatz_conjecture(mut number: u32) {

    loop {
        if number == 1 {
            print!("1");
            break;
        }
        
        let next_number;

        if number%2 == 0 {
            next_number = number/2;

        } else {
            next_number = number*3 + 1;
        }

        print!("{} -> ", number);
        number = next_number;

    }
}
fn main () {
    
    println!("--Insert the number");
    let mut number_str = String::new();
    io::stdin().read_line(&mut number_str).expect("Invalid value");
    let number_clean: u32 = number_str.trim().parse().expect("error");

    println!("---RESULTS---");
    collatz_conjecture(number_clean);

}