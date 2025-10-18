use std::io;

fn summer (starter: i32, endless: i32) {
    let mut summ = 0;
    for i in starter..=endless {
        summ = summ + i;
    }

    println!("The summ of all numbers is {}", summ);
   
}

fn main () {

    println!("--INSERT THE START OF SUMM--");
        let mut first_str = String::new();
        io::stdin().read_line(&mut first_str).expect("Invalid value");
            let first: i32 = first_str.trim().parse().expect("Invalid value");
    println!("--INSERT THE FINAL OF SUMM--");
        let mut second_str = String::new();
        io::stdin().read_line(&mut second_str).expect("Invalid value");
            let second: i32 = second_str.trim().parse().expect("Invalid value");

    summer(first, second);

}