use std::io;

    const DISCOUNT_FOR_MANS: f64 = 0.95;
    const DISCOUNT_FOR_WOMANS: f64 = 0.87;
    fn main (){
        println!("What is your name?");
            let mut name = String::new();
            io::stdin().read_line(&mut name)
            .expect("Enter a valid name");

        println!("write down how much money you spent");
            let mut money_writed = String::new();
            io::stdin().read_line(&mut money_writed)
            .expect("enter a valid value");

            let money: f64 = money_writed.trim().parse().expect("msg");
        
        println!("What is your sex (insert M for man and W for woman)");
            let mut sex_writed = String::new();
            io::stdin().read_line(&mut sex_writed)
            .expect("Enter a valid sex");

            let sex_input = sex_writed.trim().to_lowercase();
            let sex = sex_input.chars().next().unwrap_or(' ');

        if sex == 'm' {

            println!("Hello {}, you will have to pay U${}", name.trim(), money*DISCOUNT_FOR_MANS);

        } else if sex == 'w'{
            
            println!("Hello {}, you will have to pay U${}", name.trim(), money*DISCOUNT_FOR_WOMANS);

        } else {
            
            println!("Enter a valid sex");

        }
        
    }