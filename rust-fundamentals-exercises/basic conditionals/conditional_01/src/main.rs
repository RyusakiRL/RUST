use std::io;

        const FINE_PER_KM: f64 = 5.0;

fn main (){

    println!("Write the velocity of car");
        let mut velocity = String::new();
        io::stdin().read_line(&mut velocity)
        .expect("Enter a valid value");

        let velocityy: f64 = velocity.trim().parse().expect("msg");

    if velocityy > 80.0 {
        
        println!("You are fined in U${}", (velocityy-80.0)*FINE_PER_KM);

    }  else {
        println!("You are ok");
    }

}