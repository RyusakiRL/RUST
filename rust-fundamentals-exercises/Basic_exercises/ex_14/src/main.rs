use std::io;

        const RENT_DAY: f64 = 90.0;
        const PRICE_KM: f64 = 0.2;

fn main(){
    println!("Write how many days the person kept the car");
            let mut x = String::new();
        io::stdin().read_line(&mut x)
        .expect("Please write in number");

    println!("Write the distance traveled in km");
            let mut y = String::new();
        io::stdin().read_line(&mut y)
        .expect("Please enter a valid number");
    

            let xx: f64 = x.trim().parse().expect("msg");
            let yy: f64 = y.trim().parse().expect("msg");

            let finale_some: f64 = RENT_DAY*xx + PRICE_KM*yy;

    println!("The final price is U${}",finale_some);

}