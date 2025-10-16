use rand::prelude::*;

fn main () {

    let mut vectorios = Vec::new();
    let mut randomizer = rand::rng();
    for _ in 1..20 {
        
    let random = randomizer.random_range(0..=99);

        vectorios.push(random);

    }
    
    println!("--Results--");
    println!("The vector original {:?}", vectorios);
    
    vectorios.sort();

    println!("In ascending order {:?}", vectorios);
    println!("--Finish--");
}