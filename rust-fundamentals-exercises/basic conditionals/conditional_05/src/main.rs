use std::io;

fn main (){
    println!("Write the year");
    let mut year_writed = String::new();
    io::stdin().read_line(&mut year_writed)
    .expect("Enter a valid number");

        let year: i32 = year_writed.trim().parse().expect("msg");

    if year%4 == 0 {

        println!("It's a leap year");
        
    } else {
        
        println!("It is not a leap year");

    }


}