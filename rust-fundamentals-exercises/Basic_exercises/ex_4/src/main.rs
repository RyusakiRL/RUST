use std::io;
    fn main (){
        println!("Enter the first number");

        let mut x = String::new();
        io::stdin().read_line(&mut x)
        .expect("msg");

        println!("Enter the second number");

        let mut y = String::new();
        io::stdin().read_line(&mut y)
        .expect("msg");

        let xx: i32 = x.trim().parse().expect("msg");
        let yy: i32 = y.trim().parse().expect("msg");

        println!("The value of some is {:.}", xx+yy);

    }