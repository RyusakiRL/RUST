use std::io;
    const DISTANCE_MORE : f64 = 0.45;
    const  DISTANCE_MINOR : f64 = 0.5;

    fn main (){
        println!("How many kilometers do you want to travel?");
        let mut km_writed = String::new();
        io::stdin().read_line(&mut km_writed)
        .expect("Type a value in number");

            let km: f64 = km_writed.trim().parse().expect("msg");
        if km<=200.0{
            println!("Ticket price is {}", km*DISTANCE_MINOR);

        } else {
            println!("Ticket price is {}", km*DISTANCE_MORE);

        }
    }