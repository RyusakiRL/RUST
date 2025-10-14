use std::io;

fn main () {

    let mut summ: f64 = 0.0;
    let mut smaller: f64 = f64::MAX;
    let mut pair = 0;
    let mut quantity: f64 = 0.0;
    loop {

        println!("--Insert Number--");
            let mut number_str = String::new();
            io::stdin().read_line(&mut number_str).expect("Insert a number");
            let number: f64 = number_str.trim().parse().expect("Not found");

        println!("--You want continue YES or NO--");
            let mut booolea = String::new();
            io::stdin().read_line(&mut booolea).expect("Insert Yes or no");
            let bolean = booolea.trim().to_lowercase();
        quantity += 1.0;
        summ += number;

        if number<smaller {
            smaller = number;
        }

        if number%2.0 == 0.0{
            pair+= 1;
        }

        if bolean == "no" {
            println!("---RESULTS---");
            println!("The summ is: [{}]", summ );
            println!("Smaller value: [{}]", smaller);
            println!("Average of all values: [{}]", summ/quantity);
            println!("How many values is pair: [{}]", pair);
            break;
        }
    }


}