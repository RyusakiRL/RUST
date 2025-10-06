use std::io;

        const DAY_IN_MINUTES: f64 = 1440.0;
        const DAYS_IN_YEARS: f64 = 365.0;
        const LIFE_LOST_PER_CIGARETE: f64 = 10.0;

    fn main (){
        
        println!("How many cigarettes do you smoke a day?");
        let mut cigarete = String::new();
        io::stdin().read_line(&mut cigarete)
        .expect("Type a valid value in number");

        println!("How many years have you been smoking?");
        let mut year_smoke = String::new();
        io::stdin().read_line(&mut year_smoke)
        .expect("Type a valid value in number");

            let cigaretes: f64 = cigarete.trim().parse().expect("error");
            let years_smokes: f64 = year_smoke.trim().parse().expect("error");

            let life_lost: f64 = (cigaretes*LIFE_LOST_PER_CIGARETE*years_smokes*DAYS_IN_YEARS)/DAY_IN_MINUTES;

        println!("Your lifespan has been reduced by {:.2} days", life_lost);

    }