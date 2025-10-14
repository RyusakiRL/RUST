use std::io;

fn main () {

    println!("Write the first number of progression");
        let mut first_str = String::new();
        io::stdin().read_line(&mut first_str).expect("Insert a number");
            let first: i64 = first_str.trim().parse().expect("It's not a number");

    println!("Write the ratio of arithmetic progression");
        let mut ratio_str = String::new();
        io::stdin().read_line(&mut ratio_str).expect("Invalid number");
            let ratio: i64 = ratio_str.trim().parse().expect("It's not a number");

        let mut summ = 0;


    for i in (first..=(first+9*ratio)).step_by(ratio.try_into().unwrap())  {
        println!("[{}]", i);
        summ += i;
    }
        println!("Summ is: [{}]",summ );
}