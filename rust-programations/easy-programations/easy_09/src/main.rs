use std::io;

fn main () {
        println!("Insert the number");
            let mut number_str = String::new();
            io::stdin().read_line(&mut number_str).expect("Invalid value");
            let mut number_clean:i32 = number_str.trim().parse().expect("Invalid value");
        
            let mut vectorios = Vec::new();
            let mut index = 0;

    loop {
        index = index +1;
       
        if number_clean%2 ==0{
            number_clean = number_clean/2;
            vectorios.push(2);
        };
        if number_clean%3 == 0 && number_clean%2!=0 {
            number_clean = number_clean/3;
            vectorios.push(3);
        };
        if number_clean%5 == 0 && number_clean%2!=0 && number_clean%3!=0 {
            number_clean = number_clean/5;
            vectorios.push(5);
        };
        if number_clean%7==0 && number_clean%5!= 0 && number_clean%3!=0 && number_clean%2!=0 {
            number_clean = number_clean/7;
            vectorios.push(7);
        };

        if number_clean == 1 {
            break;
        };
    }

        print!("Prime factors : [");

        for i in 0..vectorios.len() {

            print!(" {} ", vectorios[i]);
        }

        println!("]");

}