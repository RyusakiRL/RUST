use std::io;

fn fibonacci (end: i32) {

    let mut before_1 = 1;
    let mut before_2 = 1;

    print!("{} {} ", 1, 1);

    for _ in 2..end {

        let mut after = before_1 + before_2;

        print!("{} ", after);

        before_2 = before_1;
        before_1 = after;
    }
}

fn main () {

    println!("--Insert how many times you want see the Fibonacci--");
        let mut time_str = String::new();
        io::stdin().read_line(&mut time_str).expect("Invalid number");
            let time: i32 = time_str.trim().parse().expect("Invalid value");
    
    fibonacci(time);
}