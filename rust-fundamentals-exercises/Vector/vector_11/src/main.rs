use std::io;

fn main () {

    let mut vectorios = Vec::new();
    let mut summ = 0.0;
    let bigger = 0.0;

    for _ in 0..=7 {
        
        println!("--Insert the age--");
            let mut age_str = String::new();
            io::stdin().read_line(&mut age_str).expect("Invalid number");
                let age: f32 = age_str.trim().parse().expect("Invalid number");

        summ+=age;

        vectorios.push(age);
      
    }
        println!("The average age is: [{}]", summ/8.0);
    for i in 0..=7 {
        
    if vectorios[i]>25.0 {
        println!("Have people over twenty five in position [{}]", i);
    }
            
    }

    for n in 0..=7 {
    
    if vectorios[n]>bigger {
        println!("The biggest age is: [{}]", vectorios[n]);
        println!("The position of biggest age is: [{}]", n);
        break;
    }
    }

}