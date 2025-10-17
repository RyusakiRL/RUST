use std::io;

fn bigger (number: i32) {
    let mut max = 0;

    if number>max {
        max = number;
    }
    println!("The bigger number is {}", max);
}

fn main () {
    let mut number_clean = 0;
    for _ in 1..=3 {
        println!("--Insert the number--");
        let mut number_str = String::new();
        io::stdin().read_line(&mut number_str).expect("Invalid number");
            number_clean = number_str.trim().parse().expect("Incorrect number");

        
    }
        bigger(number_clean);
    
    
    
}