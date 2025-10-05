use std::io;

const REAL_TO_DOLAR: f64 = 0.19;
fn main(){
    println!("Write the quantity of money in R$");
   
   
    let mut m = String::new();
        io::stdin().read_line(&mut m)
        .expect("msg");
   
   
    let mm: f64 = m.trim().parse().expect("msg");

        println!("You have R${} or U${}", mm, mm*REAL_TO_DOLAR );
}