use std::io;

fn main() {
  
    let mut age_writed: f64 = 0.0;
    let mut overtwenty: u32 = 0;
    let mut summ = 0.0;

   loop {
       
    println!("--Insert the age--");
        let mut age_str = String::new();
        io::stdin().read_line(&mut age_str).expect("Don't possible the read");
            let age: f64 = age_str.trim().parse().expect("Not is number");
   
    println!("--You want continue YES or NO--");
        let mut boolean = String::new();
        io::stdin().read_line(&mut boolean).expect("Insert yes or no");
            let bolean = boolean.trim().to_lowercase();

    if age>21.0 {
        overtwenty += 1;
    }
        summ += age;
        age_writed += 1.0;

        if bolean == "no" {
            println!("Ages writed: [{}]", age_writed);
            println!("Average age: [{}]",summ/age_writed);
            println!("Peoples over 21: [{}]", overtwenty);
        break;       
        }
   
   }

}