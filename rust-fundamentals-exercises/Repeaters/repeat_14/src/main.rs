use std::io;

    fn main () {

        let mut limit = 0;
        let mut maior: f64 = 0.0;
        let mut menor: f64 = f64::MAX;


    while limit<8 {
        println!("---Insert the price---");
        let mut number = String::new();
        io::stdin().read_line(&mut number);

        limit = limit + 1;

        let numbers: f64 = number.trim().parse().expect("msg");

        if numbers>maior{
            maior = numbers;
        } 

        if numbers<menor {
            menor = numbers;
        }
        
        }

        println!("---Results---");
        println!("The biggest price is: U${:.2}", maior);
        println!("The biggest price is: U${:.2}", menor);
    }
