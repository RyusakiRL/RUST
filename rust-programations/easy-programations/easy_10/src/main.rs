use std::i32::MAX;
use std::io;
use rand::Rng;

fn main () {

    println!("Insert how many raindrops you want");
        let mut limiter_str = String::new();
        io::stdin().read_line(&mut limiter_str).expect("Invalid value");
        let limit: i32 = limiter_str.trim().parse().expect("Invalid value");

        let mut vectorios: Vec<String> = Vec::new();

        print!("The results is: [");

    for i in 0..limit {
        
        let mut random = rand::rng();
        let rng = random.random_range(1..MAX);

        if rng%3 == 0 {
            vectorios.push("Pling".to_string());
        }

        if rng%5 == 0 && rng%3!=0 {
            vectorios.push("Plang".to_string());
        }

        if rng%7 == 0 && rng%5!=0 && rng%3!=0{
            vectorios.push("Plong".to_string());
        }

        if rng%7!=0 && rng%5!=0 && rng%3!=0 {

            let rng_str: String = rng.to_string();
            vectorios.push(rng_str);
        }
        
        let index_i: usize = i.try_into().unwrap();

        print!("{} ", vectorios[index_i]);

    }
        println!("]");
        


}