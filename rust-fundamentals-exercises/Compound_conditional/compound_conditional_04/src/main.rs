use std::io;

    const WORK_THREEY: f64 = 1.03;
    const WORK_TENY: f64 = 1.2;
    const WORK_BETWEEN: f64 = 1.125;
    fn main (){
        println!("What is your name?");
        let mut name = String::new();
        io::stdin().read_line(&mut name)
        .expect("Insert a valid name");

        println!("What is your salary?");
        let mut salary_str = String::new();
        io::stdin().read_line(&mut salary_str)
        .expect("Insert a valid number");

        println!("How long have you worked at the company?");
        let mut year_str = String::new();
        io::stdin().read_line(&mut year_str)
        .expect("Insert a valid number");

            let salary: f64 = salary_str.trim().parse().expect("it's not a valid number");
            let year: f64 = year_str.trim().parse().expect("It's not a valid number");
        if year<=3.0 {
            println!("Hello {}, good work your new salary is U${}", name.trim(), salary*WORK_THREEY);
    
        } else if 3.0<year && year<10.0 {
            println!("Hello {}, good work your new salary is U${}", name.trim(), salary*WORK_BETWEEN);
        
        } else if year>=10.0 {
            println!("Hello {}, good work your new salary is U${}", name.trim(), salary*WORK_TENY);
       
        } else {
            println!("Invalid valors");
        }
    }