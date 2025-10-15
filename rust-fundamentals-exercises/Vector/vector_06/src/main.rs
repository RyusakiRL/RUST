use rand::prelude::*;

fn main () {

    let mut randomizer = rand::rng();
    let mut vectorios = Vec::new();

    for i in 0..=6 {

        let random = randomizer.random_range(0..u128::MAX);

        vectorios.push(random);       

        println!("Position {} filled by {}", i, vectorios[i]);
    }
        println!("Final vector ({} positions)", vectorios.len());
        println!("{:?}", vectorios);
    
}