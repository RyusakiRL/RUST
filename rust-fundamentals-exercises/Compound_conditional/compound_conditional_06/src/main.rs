use std::io;
    fn main () {
            println!("What is your height?");
            let mut height_str = String::new();
            io::stdin().read_line(&mut height_str).expect("msg");

            println!("What is your weight");   
            let mut weight_writed = String::new();
            io::stdin().read_line(&mut weight_writed).expect("msg");

                let height: f64 = height_str.trim().parse().expect("msg");
                let weight: f64 = weight_writed.trim().parse().expect("msg");

                let imc: f64 = weight/(height*height);
        
        if imc<18.5 {
            println!("Underweight {}", imc);
        } else if 18.5<=imc && imc<30.0{
            println!("Ideal weight {}", imc);
        } else if 30.0<=imc && imc<40.0 {
            println!("Overweight {}", imc);
        } else if imc>=40.0 {
            println!("Morbid obesity");
        }
}