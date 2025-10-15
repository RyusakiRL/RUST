fn main() {
   
    let mut vectorios = Vec::new();
    let mut number = 0;

    for i in 0..=9 {

    if i%2 == 0 {
        number = 5;
    } else {
        number = 3;
    }
    
        vectorios.push(number);

        println!("Position {} filled by {}", i, vectorios[i])
    }

    println!("Final vector ({} positions)", vectorios.len());
    println!("{:?}", vectorios);
   
}
