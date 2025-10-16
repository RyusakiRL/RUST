use std::io;

fn main () {

    let mut roronoazoro = Vec::new();
    let mut vinsmokesanji = Vec::new();
    let mut monkeydluffy = Vec::new();

    for _ in 0..5 {

        println!("--Insert the name--");
            let mut name_str = String::new();
            io::stdin().read_line(&mut name_str).expect("Invalid name");
            let name_clean = name_str.trim().to_string();

        println!("--Insert the gender (Man or Woman)--");
            let mut gender_str = String::new();
            io::stdin().read_line(&mut gender_str).expect("Invalid gender");
            let gender_clean = gender_str.trim().to_string();

        println!("--Insert the salary--");
            let mut salary_str = String::new();
            io::stdin().read_line(&mut salary_str).expect("Invalid number");
            let salary: f64 = salary_str.trim().parse().expect("Invalid number");

    roronoazoro.push(name_clean);
    vinsmokesanji.push(gender_clean);
    monkeydluffy.push(salary);

    }

        println!("-------Results-------");
    for i in 0..roronoazoro.len() {
    
    if monkeydluffy[i]>5000.00 {
        println!("{} is a {} who earns U${:.2}", roronoazoro[i], vinsmokesanji[i], monkeydluffy[i] );
    }
    }
}