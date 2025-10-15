use std::io;

fn main () {

    let mut vectorios = Vec::new();

    for _ in 1..=14 {
        
        println!("--Insert number--");
            let mut number_str = String::new();
            io::stdin().read_line(&mut number_str).expect("Invalid number");
                let number: u64 = number_str.trim().parse().expect("Failed in read the number");
            
        vectorios.push(number);

    }

        

    for i in 1..=14 {
        
        println!("Position {} filled by {}", i, vectorios[i]);

    if vectorios[i]%10 == 0 {
        println!("In position {} have a multiple of ten", i)
    }
    }

}