use std::io;

fn main () {

    
    let mut people = 0.0;
    let mut eighteen = 0.0;
    let mut minusfives = 0.0;
    let mut maior: f64 = 0.0;
    let mut somatorie: f64 = 0.0;

    while people<10.0 {
        println!("---Insert the age---");
        let mut people_age = String::new();
        io::stdin().read_line(&mut people_age).expect("msg");

            let age: f64 = people_age.trim().parse().expect("msg");
            
            people = people + 1.0;
            somatorie = somatorie + age;

        if age>18.0 {
            eighteen = eighteen + 1.0;
        }

        if age<5.0 {
            minusfives = minusfives + 1.0;
        }

        if age>maior {
            maior = age;
        }

        
    }

        println!("---Results---");
        println!("People of legal age: [{}]", eighteen);
        println!("People under five years of age: [{}]", minusfives);
        println!("Older person: [{}]", maior);
        println!("Average age: [{}]", somatorie/10.0 );
}