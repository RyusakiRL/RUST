use std::io;

    fn main(){
        println!("What is your name?");
        let mut x = String::new();
        io::stdin().read_line(&mut x)
        .expect("msg");

        println!("What is your salary?");
        let mut y = String::new();
        io::stdin().read_line(&mut y)
        .expect("msg");

        let yy: f64 = y.trim().parse().expect("msg");


        println!("Good evening {:.} your salary is {:.}", x.trim(), yy);

    }