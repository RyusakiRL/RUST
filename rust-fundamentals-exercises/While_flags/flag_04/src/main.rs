use std::io;

fn main () {

    let mut bigger: f64 = 0.0;
    let mut smaller: f64 = f64::MAX;
    let mut quantity_man = 0.0;
    let mut summ_man = 0.0;

    loop {
        
        println!("--Insert your gender man or woman--");
            let mut gender_str = String::new();
            io::stdin().read_line(&mut gender_str).expect("msg");
                let gender = gender_str.trim().to_lowercase();

        println!("--Insert the age--");
            let mut age_str = String::new();
            io::stdin().read_line(&mut age_str).expect("msg");
                let age: f64 = age_str.trim().parse().expect("msg");

        println!("--You want continue YES or NO--");
            let mut boolean = String::new();
            io::stdin().read_line(&mut boolean).expect("msg");
                let bolean = boolean.trim().to_lowercase();
        
        if gender == "man" {
            quantity_man = quantity_man + 1.0;
        }

        if gender == "woman" && age<smaller  {
            smaller = age;
        }

        if gender == "man" {
            summ_man = summ_man + age;
        }

        if age>bigger {
            bigger = age;
        }

        if bolean == "no" {
            break;
        }
    }

    println!("--Results--");
    println!("The bigger age is: [{}]", bigger);
    println!("Number of registered men: [{}]", quantity_man);
    println!("Age of the youngest woman: [{}]", smaller);
    println!("Average age among men: [{}]", summ_man/quantity_man);
}