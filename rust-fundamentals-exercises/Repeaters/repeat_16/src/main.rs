use std::io;

    fn main () {

        let mut quantity_man: f64 = 0.0;
        let mut quantity_woman: f64 = 0.0;
        let mut limit = 0;
        let mut somatorie: f64 = 0.0;
        let mut somatiore_man: f64 = 0.0;
        let mut over_twenty: f64 = 0.0;
        
    while limit<5 {

        println!("---Gender Woman or Man---");
            let mut gender = String::new();
            io::stdin().read_line(&mut gender).expect("msg");
        println!("--Age--");
            let mut age_str = String::new();
            io::stdin().read_line(&mut age_str).expect("msg");

                let age: f64 = age_str.trim().parse().expect("msg");
                let gender_n = gender.trim().to_lowercase();
        somatorie = somatorie + age;
        limit = limit + 1;

        if gender_n == "man" {
            quantity_man = quantity_man + 1.0;
            somatiore_man = somatiore_man + age;
        }

        if gender_n == "woman" {
            quantity_woman = quantity_woman + 1.0;
        }

        if gender_n == "woman" && age>20.0 {
            over_twenty = over_twenty + 1.0;
        }
    }
    println!("---Results---");
    println!("Quantity of mans: [{}]", quantity_man);
    println!("Quantity of womens: [{}]", quantity_woman);
    println!("Average age of group: [{}]", somatorie/5.0);
    println!("Average age of men: [{}]", somatiore_man/quantity_man);
    println!("Woman over twenty: [{}]", over_twenty)
}