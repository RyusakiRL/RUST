use std::io;

fn main (){
    println!("Write your first number");
    let mut x = String::new();

    io::stdin().read_line(&mut x)
    .expect("msg");

    let xx: i32 = x.trim().parse().expect("msg");

    println!("The number {}, have a sucessor {} and predecessor {}", xx, xx+1, xx-1);

}