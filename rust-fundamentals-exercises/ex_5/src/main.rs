use std::io;

fn main(){
    println!("Write your first test note");
    let mut x = String::new();
    io::stdin().read_line(&mut x)
    .expect("msg");

    println!("Write your second test note");
    let mut y =String::new();
    io::stdin().read_line(&mut y)
    .expect("msg");

   let xx: f64 = x.trim().parse().expect("msg");
    let yy: f64 = y.trim().parse().expect("msg");

    println!("Your average is {}", (xx+yy)/2.0);

}