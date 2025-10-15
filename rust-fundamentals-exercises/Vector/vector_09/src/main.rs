use std::io;

fn main () {

    let mut vectorios = Vec::new();

    for _ in 0..=9 {
        println!("--Insert a integer number--");
            let mut number_str = String::new();
            io::stdin().read_line(&mut number_str).expect("Invalid number string");
                let number_clean: u64 = number_str.trim().parse().expect("Invalid number");

        vectorios.push(number_clean);
    }

    for i in 0..=9 {
        if vectorios[i]%2 == 0 {
            println!("The number {} in position {} is pair", vectorios[i], i);
        }
    }

}