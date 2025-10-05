use std::io;

fn main (){
    println!("Write a number");

    let mut  x = String::new();
    io::stdin().read_line(&mut x)
    .expect("msg");

    let xx: f64 = x.trim().parse().expect("msg");

    println!("The double of {} is {} and your thid part is {}", xx, xx*2.0, xx/3.0);

}