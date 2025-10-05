use std::io;

fn main (){

        println!("Write the width of a wall");
    let mut w = String::new();
    io::stdin().read_line(&mut w)
    .expect("msg");

        println!("Write the height of the same wall");
    let mut h = String::new();
    io::stdin().read_line(&mut h)
    .expect("msg");

            let ww: f64 = w.trim().parse().expect("msg");
            let hh: f64 = h.trim().parse().expect("msg");

            let a = hh*ww;
        println!("Area of wall is {}, necesssary {} bucket of ink", a, a/2.0 );
}