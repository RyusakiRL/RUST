use std::io;

fn main () {
        println!("Insert the first number of progression");
        let mut first_writed = String::new();
        io::stdin().read_line(&mut first_writed).expect("msg");

        println!("Insert the ratio of progression");
        let mut ratio_writed = String::new();
        io::stdin().read_line(&mut ratio_writed).expect("msg");

            let mut first: i32 = first_writed.trim().parse().expect("msg");
            let ratio: i32 = ratio_writed.trim().parse().expect("msg");

                let mut number = 0;

    while number<=6 {
        println!("{}", first);

        number = number + 1;
        first = first + ratio;
    }
        println!("End!!");
}