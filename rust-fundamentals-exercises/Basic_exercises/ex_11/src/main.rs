use std::io;

    fn main(){
        println!("Write the value of first variable");
        let mut a = String::new();
        io::stdin().read_line(&mut a)
        .expect("msg");

        println!("Write the value of second variable");
        let mut b = String::new();
        io::stdin().read_line(&mut b)
        .expect("msg");

        println!("Write the value of third variable");
        let mut c = String::new();
        io::stdin().read_line(&mut c)
        .expect("msg");

            let aa: f64 = a.trim().parse().expect("msg");
            let bb: f64 = b.trim().parse().expect("msg");
            let cc: f64 = c.trim().parse().expect("msg");

        let delta = bb*bb -4.0*aa*cc;

            println!("The value of delta of the quadratic equation {}", delta);
    }