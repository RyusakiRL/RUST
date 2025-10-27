use time::{Duration, PrimitiveDateTime};
use time::format_description;
use std::io;

const GIGASECOND: Duration = Duration::seconds(1_000_000_000);

pub fn after (start: PrimitiveDateTime) -> PrimitiveDateTime {

    start + GIGASECOND

}

fn main () {
    println!("---GIGASECOND CALCULATOR---");
    println!("Insert initial date (ex: 2015-01-24 22:00:00)");

    let mut datestr = String::new();
    io::stdin().read_line(&mut datestr).expect("Error in the enter");

    let date_clean = datestr.trim();

    let format = format_description::parse(
        "[year]-[month]-[day] [hour]:[minute]:[second]"
    ).expect("error");


    match PrimitiveDateTime::parse(date_clean, &format) {
        Ok(start_date) =>{
            let one_gigasecond_later = after(start_date);

            println!("Start date {}", date_clean);
            println!("Date after one gigasecond {}", one_gigasecond_later);
        }
    Err(e) => {
        eprintln!("Invalid date, insert in format");
    }
    }

        
}