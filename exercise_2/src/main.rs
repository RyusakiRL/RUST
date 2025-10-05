use std::io;

    fn main(){
        println!("What is your name?");

        let mut x = String::new();
        io::stdin().read_line(&mut x)
        .expect("msg");

        println!("My name is {}", x);
        
    }