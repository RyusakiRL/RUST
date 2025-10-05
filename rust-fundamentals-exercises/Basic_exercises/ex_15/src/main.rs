use std::io;

    const SALARY_PER_HOUR: f64 = 25.0;
    fn main() {
        println!("How many days you worked in this month?");
                let mut day = String::new();

                io::stdin().read_line(&mut day)
                .expect("Type integer value");

        println!("How many hours do you work?");
                let mut hour = String::new();

                io::stdin().read_line(&mut hour)
                .expect("Type a float value");

            let dayi: f64 = day.trim().parse().expect("msg");
            let houri: f64 = hour.trim().parse().expect("msg");
            
                let final_salary: f64 = dayi*houri*SALARY_PER_HOUR;

        println!("Your salary is U${:.2}", final_salary);


    }