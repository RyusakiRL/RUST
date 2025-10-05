use std::io;

const M_TO_CM: f64 = 100.0;
const M_TO_MM: f64 = 1000.0;
const M_TO_DM: f64 = 10.0;
const M_TO_KM: f64 = 1.0/1000.0;
const M_TO_DAM: f64 = 1.0/10.0;
const M_TO_HM: f64 = 1.0/100.0;

fn main(){
    println!("Write a distance in meters");
        let mut m = String::new();
    io::stdin().read_line(&mut m)
    .expect("msg");

        let mm: f64 = m.trim().parse().expect("msg");

    println!("The value of {} meters in centimers is {}, in milimiters {}", mm, mm*M_TO_CM, mm*M_TO_MM);

    println!("in kilometers {}, in decimeters {}, in decameters {}", mm*M_TO_KM, mm*M_TO_DM, mm*M_TO_DAM);

    println!("in hectometers {}", mm*M_TO_HM);


}