use std::io;

    fn main () {
            println!("Insert a Natural Number");
            let mut number_str = String::new();
            io::stdin().read_line(&mut number_str).expect("msg");

                let number = number_str.trim().parse().expect("msg");
                let mut variable = 0;

            while variable!=number {
                variable = variable + 1;
                
                println!("{}", variable);
            }
                println!("End!!");
    }