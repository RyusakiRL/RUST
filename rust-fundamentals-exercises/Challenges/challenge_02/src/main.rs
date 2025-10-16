use std::io;

    fn main (){
        println!("Write the first value of your triangle");
            let mut a_writed = String::new();
            io::stdin().read_line(&mut a_writed)
            .expect("Type a valid value");

        println!("Write the second value of your triangle");
            let mut b_writed = String::new();
            io::stdin().read_line(&mut b_writed)
            .expect("Type a valid value");

        println!("Write the third value of your triangle");
            let mut c_writed = String::new();
            io::stdin().read_line(&mut c_writed)
            .expect("Type a valid value");

                let a: f64 = a_writed.trim().parse().expect("msg");
                let b: f64 = b_writed.trim().parse().expect("msg");
                let c: f64 = c_writed.trim().parse().expect("msg");

                    let some_b_c = b+c;
                    let some_a_c = a+c;
                    let some_a_b = a+b;
        
        if a<some_b_c && b<some_a_c && c<some_a_b{
            println!("The triangle exist");
        } else {
            println!("The triangle don't exist");
        }
    }