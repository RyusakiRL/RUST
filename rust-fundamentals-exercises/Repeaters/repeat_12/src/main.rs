use std::io;

    fn main() {

            println!("Insert the first number of progression");
            let mut primeir_numero = String::new();
            io::stdin().read_line(&mut primeir_numero).expect("msg");

            println!("Insert the ratio of progression");
            let mut ratio_str = String::new();
            io::stdin().read_line(&mut ratio_str).expect("error");
            
            let ratio: i32 = ratio_str.trim().parse().expect("msg");
            let mut primeiro: i32 = primeir_numero.trim().parse().expect("msg");
            let mut par = 0;
            let mut impar = 0;
            let mut limit = 0;

        while limit<6 {
           
            println!("[{}]", primeiro);
            primeiro = primeiro + ratio;
            limit = limit + 1;
        
            if primeiro%2 == 0 {
                par = par + 1;
            } else {
                impar = impar + 1;
            }
        
        }
            println!("The number of pairs is {}", par);
            print!("The number of odd numbers is {}", impar);

    }