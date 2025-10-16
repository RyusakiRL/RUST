use std::io;

fn main () {

    let mut vectorios = Vec::new();
    let mut vectoryu = Vec::new();
    
    for _ in 0..=8 {
        println!("--Insert the name--");
            let mut name_str = String::new();
            io::stdin().read_line(&mut name_str).expect("Invalid name");
                let name_clean = name_str.trim().to_string();

        println!("--Insert the age--");
            let mut age_str = String::new();
            io::stdin().read_line(&mut age_str).expect("Invalid number");
                let age: u32 = age_str.trim().parse().expect("Invalid number");
        
        vectorios.push(name_clean);
        vectoryu.push(age);
    }

        println!("----Results----");

    for i in 0..vectorios.len() {
        if vectoryu[i]<18 {

            println!("The {} is a minor.", vectorios[i]);
        }

    }

}