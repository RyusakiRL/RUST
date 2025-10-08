use std::io;

    const MONTH_IN_YEARS: f64 = 12.0;
    const EXCEED: f64 = 0.3;

fn main (){
        println!("What is the value of the house?");
        let mut valueh_writed = String::new();
        io::stdin().read_line(&mut valueh_writed).expect("msg");

        println!("What is your salary?");
        let mut salary_writed = String::new();
        io::stdin().read_line(&mut salary_writed).expect("msg");

        println!("How long will it take you to pay?");
        let mut time_waste = String::new();
        io::stdin().read_line(&mut time_waste).expect("msg");

            let valueh: f64 = valueh_writed.trim().parse().expect("msg");
            let salary: f64 = salary_writed.trim().parse().expect("msg");
            let time: f64 = time_waste.trim().parse().expect("msg");

    if salary*EXCEED<valueh/(time*MONTH_IN_YEARS) {
        println!("You are poor, you don't have money for buy a house");
    } else {
        println!("The house payment is per month U${}", valueh/(time*MONTH_IN_YEARS));
    }
}
