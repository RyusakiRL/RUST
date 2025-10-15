fn main() {
    let mut vectorios = Vec::new();
    let mut initial_number = 0;
    let mut multiplier = 0;

    for i in 0..=9 {
             
        multiplier += 1;
        initial_number = 5*multiplier;

        vectorios.push(initial_number);

        println!("Position {} the number is {}", i, vectorios[i]);

    }

    println!("Final vector ({} positions)", vectorios.len());
    println!("{:?}", vectorios);


}
