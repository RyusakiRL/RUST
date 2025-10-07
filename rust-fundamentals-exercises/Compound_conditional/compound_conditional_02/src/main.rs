use std::io;

    const FAILED: f64 = 4.9;
    const APPROVED: f64 = 7.0;

    fn main(){
        println!("Register your first result of exam");
            let mut first_str = String::new();
            io::stdin().read_line(&mut first_str)
            .expect("Insert a valid value");

        println!("Register your second result of exam");
            let mut second_str = String::new();
            io::stdin().read_line(&mut second_str)
            .expect("Insert a valid value");

                let first: f64 = first_str.trim().parse().expect("msg");
                let second: f64 = second_str.trim().parse().expect("msg");

                let media = (first+second)/2.0;

        if media>=APPROVED && media<=10.0 {
            println!("Approved, congratulations");
        } else if media<=FAILED {
            println!("You failed, study more");
        } else if 5.0<=media && media<=6.9 {
            println!("Exam, try a little harder");
        } else {
            println!("Enter a valid number from 0 to 10");
        }
            
    }