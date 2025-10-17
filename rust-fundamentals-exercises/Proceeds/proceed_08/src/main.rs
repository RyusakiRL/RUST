use std::io;

fn coutage (start: i32, end: i32, ratio: i32) {
    let mut multiplier = 0;
    let mut current_term = start;
if start<end {
    while current_term<end {
        multiplier = multiplier + 1;
        current_term = start+(ratio*multiplier);
        println!("[{}]", start+(ratio*multiplier));
    }
}
if start>end {
    while current_term>end {
        multiplier = multiplier + 1;
        current_term = start+(ratio*multiplier);
        println!("[{}]", start+(ratio*multiplier));
    }
}
}

fn main () {

    println!("---INSERT THE START OF PROGRESSION---");
        let mut star_str = String::new();
        io::stdin().read_line(&mut star_str).expect("Invalid number");
            let startc: i32 = star_str.trim().parse().expect("Invalid value");

    println!("---INSERT THE END OF PROGRESSION---");
        let mut end_str = String::new();
        io::stdin().read_line(&mut end_str).expect("Insert a number");
            let endless: i32 = end_str.trim().parse().expect("invalid value");

    println!("---INSERT THE RATIO OF PROGRESSION---");
        let mut ratio_str = String::new();
        io::stdin().read_line(&mut ratio_str).expect("Invalid value");
            let ratios: i32 = ratio_str.trim().parse().expect("Insert a number");

    coutage(startc, endless, ratios);
}