fn main() {
    let mut vectorios = Vec::new();
    let mut reversal = 10;

    
    for i in 0..=9 {
        
        reversal = reversal - 1;
        vectorios.push(reversal);
        println!("The position {} is filled by {}", i, vectorios[i])
  }
        println!("Final vector ({} positions)", vectorios.len());
        println!("{:?}", vectorios);

}
