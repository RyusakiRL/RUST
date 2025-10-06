use std::io;

    const AGE_OF_ALISTMENT: i32 = 18;
    fn main () {

        println!("Your age");
        let mut age_str = String::new();
        io::stdin().read_line(&mut age_str)
        .expect("Enter a valid number");

            let age: i32 = age_str.trim().parse().expect("msg");

        if age<AGE_OF_ALISTMENT {

            println!("{} years left until enlistment", AGE_OF_ALISTMENT-age);

        } else if age == AGE_OF_ALISTMENT{
            println!("You must enlist this year")

        } else {
            
            println!("It's been {} years of enlistment", age-AGE_OF_ALISTMENT);

        }

    }