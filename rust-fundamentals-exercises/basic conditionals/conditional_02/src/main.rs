use std::io;
use chrono::{Datelike, Utc};
const VOTING_AGE: i32 = 16;

    fn main (){
        println!("What your year of birth?");
            let mut birth_year = String::new();
            io::stdin().read_line(&mut birth_year)
            .expect("Type a valid value");


        let year_of_birth: i32 = birth_year.trim().parse().expect("msg");
        let current_year: i32 = Utc::now().year();
            let age: i32 = current_year - year_of_birth;

        if VOTING_AGE<=age {

            println!("You can vote");
            
        } else {
            
            println!("You can't vote");

        }

    }