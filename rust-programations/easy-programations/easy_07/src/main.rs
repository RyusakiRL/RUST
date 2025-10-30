use std::io;

fn main() {
    println!("Insert the year");
    let mut year_str = String::new();
    io::stdin().read_line(&mut year_str).expect("Invalid value");
    let year_clean: u128 = year_str.trim().parse().expect("Invalid value");

    if year_clean%4==0 && year_clean%100!=0  {
        println!("The year {} is a leap year", year_clean);
    } else if year_clean%400== 0 {
        println!("The year {} is a leap year", year_clean);
    } else {
        println!("The year {} is not a leap year", year_clean)
    }
}
