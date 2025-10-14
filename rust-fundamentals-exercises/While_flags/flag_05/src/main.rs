use std::io;

fn main() {
    let mut bigger: u32 = 0;
    let mut smaller: u32 = 120;
    let mut named_person: String = "".to_string();
    let mut woman_younger: String = "".to_string();
    let mut quantity_people: u32 = 0;
    let mut summ: u32 = 0;
    let mut man_overthirty = 0;
    let mut woman_beloweighteen = 0;

    loop {
        println!("--Insert the name---");
        let mut name_str = String::new();
        io::stdin().read_line(&mut name_str).expect("Failed to read name");
        let name: String = name_str.trim().to_string();

        println!("--Insert the age--");
        let mut age_str = String::new();
        io::stdin().read_line(&mut age_str).expect("Failed to read age");
        let age: u32 = age_str.trim().parse().expect("Please enter a valid number");

        println!("--Insert your sex MAN or WOMAN--");
        let mut gender_str = String::new();
        io::stdin().read_line(&mut gender_str).expect("Failed to read gender");
        let gender = gender_str.trim().to_lowercase();

        quantity_people += 1;
        summ += age;

        
        if age > bigger {
            bigger = age;
            named_person = name.clone();
        }

        
        if gender == "woman" && age < smaller {
            smaller = age;
            woman_younger = name.clone();
        }

        
        if gender == "man" && age > 30 {
            man_overthirty += 1;
        }

     
        if gender == "woman" && age < 18 {
            woman_beloweighteen += 1;
        }

        println!("--You want continue YES or NO--");
        let mut bol_str = String::new();
        io::stdin().read_line(&mut bol_str).expect("Failed to read continue");
        let bol = bol_str.trim().to_lowercase();

        if bol == "no" {
            println!("---Results---");
            println!("Name of old people: {}", named_person);
            println!("Name of youngest woman: {}", woman_younger);
            let avg = summ as f32 / quantity_people as f32;
            println!("Average age of group: {:.2}", avg);
            println!("Man with age over 30: [{}]", man_overthirty);
            println!("Woman with age below 18: [{}]", woman_beloweighteen);
            break;
        }
    }
}