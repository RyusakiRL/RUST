use std::io;

fn main () {

        let mut summ_salary_man: f64 = 0.0;
        let mut summ_salary_woman: f64 = 0.0;

    loop {
        
        println!("--You are man or woman?--");
            let mut gender_str = String::new();
            io::stdin().read_line(&mut gender_str).expect("Error");
                let gender = gender_str.trim().to_lowercase();

        println!("--Insert the salary--");
            let mut salary_str = String::new();
            io::stdin().read_line(&mut salary_str).expect("msg");
                let salary: f64 = salary_str.trim().parse().expect("msg");
                
        println!("---You want to continue (yes or no)---");
            let mut bolean = String::new();
            io::stdin().read_line(&mut bolean).expect("msg");
                let bol = bolean.trim().to_lowercase();

        if gender == "man" {
            summ_salary_man = summ_salary_man + salary;
        }

        if gender == "woman" {
            summ_salary_woman = summ_salary_woman + salary;
        }

        if bol == "no" {
            break;
        }

    }

        println!("---Results---");
        println!("Total salary paid to men: U${:.2}", summ_salary_man);
        println!("Total salary paid to women: U${:.2}", summ_salary_woman);
}