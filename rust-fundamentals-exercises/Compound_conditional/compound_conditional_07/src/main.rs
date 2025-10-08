use std::io;

    const BUDGET_CAR: f64 = 90.0;
    const LUXURY_CAR: f64 = 150.0;


    fn main () {
            println!("Is the car you are rented a budget or a luxury car?");
            let mut car_writed = String::new();
            io::stdin().read_line(&mut car_writed).expect("msg");

            println!("How many days you stayed with the car?");
            let mut days_writed = String::new();
            io::stdin().read_line(&mut days_writed).expect("msg");

            println!("How many kilometers did you travel?");
            let mut km_writed = String::new();
            io::stdin().read_line(&mut km_writed).expect("msg");

                let car: String = car_writed.trim().to_lowercase().parse().expect("msg");
                let days: f64 = days_writed.trim().parse().expect("msg");
                let kms: f64 = km_writed.trim().parse().expect("msg");
        
        if car == "budget" && kms<=100.0 {
            println!("The final price is U${}", (BUDGET_CAR*days)+(kms*0.2));    
        } else if car == "budget" && kms>100.0 {
            println!("The final price is U${}", (BUDGET_CAR*days)+(kms*0.1));
        }

        if car == "luxury" && kms<=100.0 {
            println!("The final price is U${}", (LUXURY_CAR*days)+(kms*0.3));
        } else if car == "luxury" && kms>100.0 {
            println!("The final price is U${}", (LUXURY_CAR*days)+(kms*0.25));
        }

    }