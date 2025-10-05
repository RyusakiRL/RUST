use std::io;

fn main(){
        println!("Write your salary");
    let mut sal = String::new();
    io::stdin().read_line(&mut sal)
    .expect("msg");

        println!("How much of a raise will you receive (enter in decimal)?");
    let mut raise = String::new();
    io::stdin().read_line(&mut raise)
    .expect("msg");

        let sall: f64 = sal.trim().parse().expect("msg");
        let raisee: f64 = raise.trim().parse().expect("msg");

    let soma: f64 = sall+sall*raisee;

        println!("Your salary is {}, with the increases its becomes {} ", sall, soma);

}