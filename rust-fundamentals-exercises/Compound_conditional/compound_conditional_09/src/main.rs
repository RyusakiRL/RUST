use std::io;

fn main () {
    
        println!("What is your salary?");
        let mut salary_writed = String::new();
        io::stdin().read_line(&mut salary_writed).expect("msg");

        println!("You are man or woman?");
        let mut gender = String::new();
        io::stdin().read_line(&mut gender).expect("msg");

        println!("How long have you worked at the company?");
        let mut years_worked = String::new();
        io::stdin().read_line(&mut years_worked).expect("msg");

            let salary: f64 = salary_writed.trim().parse().expect("msg");
            let years: f64 = years_worked.trim().parse().expect("msg");
            let gend: String = gender.trim().to_lowercase().parse().expect("msg");

        if gend == "man" &&  years<20.0 {
            println!("Your new salary is U${}", salary*1.03);
        } else if gend == "man" && 20.0<=years && years<=30.0 {
            println!("Your new salary is U${}", salary*1.13);
        } else if gend == "man" && years>30.0{
            println!("Your new salary is U${}", salary*1.25);
        }

        if gend == "woman" && years<15.0 {
            println!("Your new salary is U${}", salary*1.05);
        } else if gend == "woman" && 15.0<=years && years<=20.0 {
            println!("Your new salary is U${}", salary*1.12);
        } else if gend == "woman" && years>20.0 {
            println!("Your new salary is U${}", salary*1.23);
        }
}