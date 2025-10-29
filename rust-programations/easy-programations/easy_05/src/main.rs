use std::io;

fn main () {
        let mut summ:i64 = 0;
        let mut vectorios = Vec::new();
        let mut summvectorios: i64 = 0;

    println!("Insert the limit of natural numbers");
        let mut limit_str = String::new();
        io::stdin().read_line(&mut limit_str).expect("Invalid value");

        let limit_clean: i64 = limit_str.trim().parse().expect("Invalid value");
    
    for i in 1..=limit_clean {
        summ = summ + i;
        vectorios.push(i);

        let index_i = i as usize;
        summvectorios = summvectorios + vectorios[index_i-1].pow(2);

    }
        println!("-----------RESULTS-----------");
        println!("The square of the sum {} and the sum of squares is {}", summ.pow(2), summvectorios);
        println!("The difference is {}", summ.pow(2)-summvectorios);



}