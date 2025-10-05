use std::io;

fn main(){

        println!("Write the value of product");
            let mut x = String::new();
            io::stdin().read_line(&mut x)
            .expect("msg");

    let xx: f64 = x.trim().parse().expect("msg");
    let discount: f64 = xx-xx*0.05;

        println!("Value of product is U${} and with discount is U${}", x.trim(), discount);
}