use std::io;

    fn main () {
        println!("Write the first lenght of your triangle");
            let mut a_str = String::new();
            io::stdin().read_line(&mut a_str)
            .expect("Insert a valid number");
        
        println!("Write second lenght of your triangle");
            let mut b_str = String::new();
            io::stdin().read_line(&mut b_str)
            .expect("Insert a valid number");

        println!("Write third lenght of your triangle");
            let mut c_str = String::new();
            io::stdin().read_line(&mut c_str)
            .expect("Insert a valid number");

                let a: f64 = a_str.trim().parse().expect("msg");
                let b: f64 = b_str.trim().parse().expect("msg");
                let c: f64 = c_str.trim().parse().expect("msg");

                    let some_a_b: f64 = a+b;
                    let some_a_c: f64 = a+c;
                    let some_b_c: f64 = b+c;
       
        if a<some_b_c && b<some_a_c && c<some_a_b {
            println!("The triangle exist");
        
        if a==b && a==c && b==c {
            println!("The triangle is equilateral");
       
        }   else if a!=b && b!=c && a!=c {
            println!("The triangle is escalene");
      
        }   else {
            println!("The triangle is isosceles");
        } 
        }   else {
            println!("The triangle don't exist");
        } 
            
}