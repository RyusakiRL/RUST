use std::io;

    fn main () {
            println!("Enter the first number of the progression");
                let mut first_writed = String::new();
                io::stdin().read_line(&mut first_writed).expect("msg");
                    let mut first: i64 = first_writed.trim().parse().expect("msg");
            
            println!("Enter the last number of the progression");
                let mut second_writed = String::new();
                io::stdin().read_line(&mut second_writed).expect("msg");
                    let second: i64 = second_writed.trim().parse().expect("msg");
                
            println!("Enter the ratio number of the progression");
                let mut ratio_writed = String::new();
                io::stdin().read_line(&mut ratio_writed).expect("msg");
                    let ratio: i64 = ratio_writed.trim().parse().expect("msg");
        
        while first<second {
                
                println!("{}", first);

                first = first + ratio;

        }
                println!("End!");
    }