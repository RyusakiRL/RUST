use std::io;

fn main () {

    let mut students: f64 = 0.0;
    let mut summ_age: f64 = 0.0;

loop {

        println!("--Insert the age--");
            let mut age_str = String::new();
            io::stdin().read_line(&mut age_str).expect("msg");
                let age: f64 = age_str.trim().parse().expect("msg");

                    students = students + 1.0;
                    summ_age = summ_age + age;

    if age == 999.0 {
        break;
    }

}
        println!("---Results---");
        println!("Quantity of students: [{}]", students-1.0);
        println!("Average o age: [{}]", (summ_age-999.0)/(students-1.0));
        
}