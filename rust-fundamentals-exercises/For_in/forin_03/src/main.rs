use std::io;

fn main() {
    println!("Enter the number to start the multiplication table");
        let mut multable = String::new();
        io::stdin().read_line(&mut multable).expect("Input a number");
            let mult: i64 = multable.trim().parse().expect("it's not a real number");

    for n in (1..=10).step_by(1) {
        
        println!("{} x {} = [{}]", mult, n, mult * n);
    }
}
